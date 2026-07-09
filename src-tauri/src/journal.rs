// 可可桌宠 v0.5 — 日记生成
use crate::db::{Database, JournalEntry};
use crate::ai::rule_engine::RuleEngine;

pub fn generate_daily(_db: &Database, affection: u32, pet_count: u32, feed_count: u32) -> String {
    let mood = RuleEngine::compute_mood(affection, 0, 12);
    let mood_text = match mood.as_str() {
        "happy" => "开心", "sleepy" => "困倦", "grumpy" => "有小脾气",
        "lonely" => "想你", "scared" => "害怕", _ => "平静"
    };
    format!("今天被摸了{}次，喂了{}次。心情{}。好感度{}。喵~期待明天！",
        pet_count, feed_count, mood_text, affection)
}

pub fn save_journal(db: &Database, content: &str, mood: &str) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let s = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let mut days = s / 86400; let mut y = 1970u32;
    loop { let dp = if y%4==0&&(y%100!=0||y%400==0){366}else{365}; if days<dp as u64 {break} days-=dp as u64; y+=1 }
    let md: [u64;12] = if y%4==0&&(y%100!=0||y%400==0){[31,29,31,30,31,30,31,31,30,31,30,31]}else{[31,28,31,30,31,30,31,31,30,31,30,31]};
    let mut m = 1u32; for v in md { if days<v {break} days-=v; m+=1 }
    let date = format!("{:04}年{:02}月{:02}日", y, m, (days+1) as u32);
    db.add_journal(&JournalEntry { date, content: content.into(), mood: mood.into() });
}
