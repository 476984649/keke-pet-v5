// 可可桌宠 v0.5 — AI 规则引擎
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest { pub input: String, pub mood: String, pub affection: u32, pub hour: u32 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse { pub text: String, pub mood_change: Option<String> }

pub struct RuleEngine;

impl RuleEngine {
    pub fn chat(req: &ChatRequest) -> ChatResponse {
        let templates = Self::templates_for_mood(&req.mood);
        let i = (req.input.len() + req.affection as usize) % templates.len();
        let picked = &templates[i];
        ChatResponse {
            text: picked.replace("{name}", "主人").replace("{affection}", &req.affection.to_string()),
            mood_change: None
        }
    }

    fn templates_for_mood(mood: &str) -> Vec<String> {
        match mood {
            "happy" => vec!["喵~今天好开心！".into(), "贴贴~".into(), "摸摸头！".into(), "主人最好啦~".into(), "呼噜呼噜~".into()],
            "sleepy" => vec!["好困哦 zzz...".into(), "哈欠~想睡觉了".into(), "眼皮好重...".into(), "zzz...".into()],
            "grumpy" => vec!["哼！".into(), "不理你了".into(), "尾巴甩甩".into(), "（扭头）".into()],
            "lonely" => vec!["你去哪了...".into(), "好想你呀".into(), "喵...（小声）".into(), "终于回来了！".into()],
            "scared" => vec!["有声音！".into(), "害怕...抖抖".into(), "躲起来...".into()],
            "excited" => vec!["好耶好耶！".into(), "鱼干！！".into(), "跑跑跳跳~".into(), "🎉".into()],
            _ => vec!["喵~".into(), "呼噜~".into(), "嗯？".into()]
        }
    }

    pub fn compute_mood(affection: u32, idle_minutes: u32, hour: u32) -> String {
        if hour >= 22 || hour <= 5 { return "sleepy".into() }
        if hour >= 13 && hour <= 15 && idle_minutes > 120 { return "sleepy".into() }
        if idle_minutes > 3600 && affection > 50 { return "lonely".into() }
        if affection < 30 { return "grumpy".into() }
        if affection > 600 { return "happy".into() }
        "happy".into()
    }

    pub fn mood_emoji(mood: &str) -> &'static str {
        match mood { "happy"=>"😊","sleepy"=>"😴","grumpy"=>"😤","lonely"=>"🥺","scared"=>"😰","excited"=>"🎉",_=>"😺" }
    }
}
