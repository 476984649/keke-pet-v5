// 可可桌宠 v0.1 — IPC 命令
use crate::db::Database;
use tauri::State;

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
pub fn add_minutes(db: State<Database>, m: u32) -> crate::db::AffectionData { db.add_minutes(m) }

#[tauri::command]
pub fn login_today(db: State<Database>) -> crate::db::AffectionData {
    use std::time::{SystemTime,UNIX_EPOCH};
    let s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let days = s/86400; let (y,m,d) = ymd(days);
    db.update_login(&format!("{:04}-{:02}-{:02}",y,m,d))
}

fn ymd(mut days: u64) -> (u32,u32,u32) {
    let mut y=1970u32; loop{let dpy=if y%4==0&&(y%100!=0||y%400==0){366}else{365};if days<dpy as u64{break}days-=dpy as u64;y+=1}
    let md:[u64;12]=if y%4==0&&(y%100!=0||y%400==0){[31,29,31,30,31,30,31,31,30,31,30,31]}else{[31,28,31,30,31,30,31,31,30,31,30,31]};
    let mut mon=1u32;for m in md{if days<m{break}days-=m;mon+=1}(y,mon,(days+1)as u32)
}
