// 可可桌宠 v0.5 — SQLite 持久化（catch_unwind 包裹）
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::panic::catch_unwind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectionData {
    pub total: u32, pub pet_count: u32, pub feed_count: u32,
    pub total_minutes: u32, pub streak_days: u32, pub last_login: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub date: String, pub content: String, pub mood: String,
}

pub struct Database { pub conn: Mutex<Connection> }

impl Database {
    pub fn open(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path).map_err(|e| format!("open: {}", e))?;
        conn.execute_batch("
            CREATE TABLE IF NOT EXISTS affection(id INTEGER PRIMARY KEY DEFAULT 1, total INTEGER DEFAULT 0, pet_count INTEGER DEFAULT 0, feed_count INTEGER DEFAULT 0, total_minutes INTEGER DEFAULT 0, streak_days INTEGER DEFAULT 0, last_login TEXT DEFAULT '');
            INSERT OR IGNORE INTO affection(id) VALUES(1);
            CREATE TABLE IF NOT EXISTS journal(id INTEGER PRIMARY KEY AUTOINCREMENT, date TEXT, content TEXT, mood TEXT DEFAULT '😊');
        ").map_err(|e| format!("batch: {}", e))?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    fn safe<T, F: FnOnce() -> T + std::panic::UnwindSafe>(f: F) -> Result<T, String> {
        catch_unwind(f).map_err(|e| format!("panic: {:?}", e))
    }

    pub fn get_affection(&self) -> AffectionData {
        Self::safe(|| {
            let conn = self.conn.lock().expect("lock");
            conn.query_row("SELECT total,pet_count,feed_count,total_minutes,streak_days,last_login FROM affection WHERE id=1", [],
                |r| Ok(AffectionData{total:r.get(0)?,pet_count:r.get(1)?,feed_count:r.get(2)?,total_minutes:r.get(3)?,streak_days:r.get(4)?,last_login:r.get(5)?}))
        }).unwrap_or_else(|_| Err(rusqlite::Error::InvalidQuery)).unwrap_or_default()
    }

    pub fn add_pet(&self) -> AffectionData {
        Self::safe(|| {
            let conn = self.conn.lock().expect("lock");
            conn.execute("UPDATE affection SET total=MIN(total+1,1000),pet_count=pet_count+1 WHERE id=1",[]).ok();
        }).ok();
        self.get_affection()
    }

    pub fn add_feed(&self, gain: u32) -> AffectionData {
        Self::safe(|| {
            let conn = self.conn.lock().expect("lock");
            conn.execute("UPDATE affection SET total=MIN(total+?1,1000),feed_count=feed_count+1 WHERE id=1",params![gain]).ok();
        }).ok();
        self.get_affection()
    }

    pub fn add_minutes(&self, m: u32) -> AffectionData {
        Self::safe(|| {
            let conn = self.conn.lock().expect("lock");
            conn.execute("UPDATE affection SET total_minutes=total_minutes+?1 WHERE id=1",params![m]).ok();
        }).ok();
        self.get_affection()
    }

    pub fn update_login(&self, today: &str) -> AffectionData {
        let aff = self.get_affection();
        Self::safe(|| {
            let conn = self.conn.lock().expect("lock");
            if aff.last_login != today {
                let streak = if aff.last_login == yesterday_of(today) { aff.streak_days + 1 } else { 1 };
                conn.execute("UPDATE affection SET streak_days=?1,last_login=?2 WHERE id=1",params![streak,today]).ok();
            }
        }).ok();
        self.get_affection()
    }

    pub fn add_journal(&self, entry: &JournalEntry) {
        Self::safe(|| {
            let conn = self.conn.lock().expect("lock");
            conn.execute("INSERT INTO journal(date,content,mood) VALUES(?1,?2,?3)",params![entry.date,entry.content,entry.mood]).ok();
        }).ok();
    }

    pub fn get_journal(&self, limit: usize) -> Vec<JournalEntry> {
        Self::safe(|| {
            let conn = self.conn.lock().expect("lock");
            let mut stmt = conn.prepare("SELECT date,content,mood FROM journal ORDER BY id DESC LIMIT ?1").expect("prepare");
            stmt.query_map(params![limit as i64],|r| Ok(JournalEntry{date:r.get(0)?,content:r.get(1)?,mood:r.get(2)?}))
                .expect("query").filter_map(|r|r.ok()).collect()
        }).unwrap_or_default()
    }
}

impl Default for AffectionData {
    fn default() -> Self { Self { total:0,pet_count:0,feed_count:0,total_minutes:0,streak_days:0,last_login:String::new() } }
}

fn yesterday_of(today: &str) -> String {
    if let Ok(parts) = today.split('-').map(|s| s.parse::<u32>()).collect::<Result<Vec<_>,_>>() {
        if parts.len()==3 {
            let (y,m,d)= (parts[0],parts[1],parts[2]);
            let dim = if m==2 { if y%4==0&&(y%100!=0||y%400==0) {29} else {28} }
                else if [4,6,9,11].contains(&m) {30} else {31};
            if d>1 { return format!("{:04}-{:02}-{:02}",y,m,d-1) }
            else if m>1 { return format!("{:04}-{:02}-{:02}",y,m-1,dim) }
            else { return format!("{:04}-12-31",y-1) }
        }
    }
    today.to_string()
}
