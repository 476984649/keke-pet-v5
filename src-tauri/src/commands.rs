// 可可桌宠 v0.5 — IPC 命令
use crate::db::Database;
use crate::ai::rule_engine::RuleEngine;
use crate::mood::Mood;
use crate::journal;
use crate::memory::MemoryStore;
use tauri::State;
use std::sync::Mutex;

pub struct AppMemory(pub Mutex<MemoryStore>);

#[tauri::command]
pub fn get_affection(db: State<Database>) -> crate::db::AffectionData { db.get_affection() }

#[tauri::command]
pub fn add_pet(db: State<Database>) -> crate::db::AffectionData { db.add_pet() }

#[tauri::command]
pub fn add_feed(db: State<Database>, food: String) -> serde_json::Value {
    let gain = match food.as_str() { "fish" => 5, "can" => 15, _ => 1 };
    let r = db.add_feed(gain);
    serde_json::json!({"total":r.total,"feedCount":r.feed_count})
}

#[tauri::command]
pub fn login_today(db: State<Database>) -> serde_json::Value {
    let s = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
    let mut days = s/86400; let mut y=1970u32;
    loop { let d=if y%4==0&&(y%100!=0||y%400==0){366}else{365}; if days<d as u64 {break} days-=d as u64; y+=1 }
    let md:[u64;12]=if y%4==0&&(y%100!=0||y%400==0){[31,29,31,30,31,30,31,31,30,31,30,31]}else{[31,28,31,30,31,30,31,31,30,31,30,31]};
    let mut m=1u32; for v in md { if days<v {break} days-=v; m+=1 }
    let today=format!("{:04}-{:02}-{:02}",y,m,(days+1)as u32);
    let a=db.update_login(&today);
    serde_json::json!({"total":a.total,"streak":a.streak_days})
}

#[tauri::command]
pub fn ai_chat(input: String, mood: String, affection: u32) -> serde_json::Value {
    let h = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()/3600%24;
    let resp = RuleEngine::chat(&crate::ai::rule_engine::ChatRequest{input,mood,affection,hour:h as u32});
    serde_json::json!({"text":resp.text,"mood":resp.mood_change})
}

#[tauri::command]
pub fn get_mood(affection: u32) -> serde_json::Value {
    let h = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()/3600%24;
    let m = Mood::compute(affection,0,h as u32);
    serde_json::json!({"mood":m.as_str(),"emoji":m.emoji()})
}

#[tauri::command]
pub fn gen_journal(db: State<Database>) -> serde_json::Value {
    let a = db.get_affection();
    let content = journal::generate_daily(&db, a.total, a.pet_count, a.feed_count);
    journal::save_journal(&db, &content, "😊");
    serde_json::json!({"content":content})
}

#[tauri::command]
pub fn get_journal(db: State<Database>) -> serde_json::Value {
    let entries = db.get_journal(10);
    serde_json::json!(entries)
}

#[tauri::command]
pub fn remember(mem: State<AppMemory>, key: String, value: String) -> serde_json::Value {
    mem.0.lock().unwrap().remember(&key, &value);
    serde_json::json!({"ok":true})
}

#[tauri::command]
pub fn recall(mem: State<AppMemory>, key: String) -> serde_json::Value {
    let v = mem.0.lock().unwrap().recall(&key).map(|s|s.to_string());
    serde_json::json!({"value":v})
}
