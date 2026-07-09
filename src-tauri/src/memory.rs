// 可可桌宠 v0.5 — 记忆系统（最多50条）
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory { pub key: String, pub value: String }

pub struct MemoryStore {
    items: VecDeque<Memory>,
    max: usize,
}

impl MemoryStore {
    pub fn new(max: usize) -> Self { Self { items: VecDeque::new(), max } }
    pub fn remember(&mut self, key: &str, value: &str) {
        if let Some(existing) = self.items.iter_mut().find(|m| m.key == key) { existing.value = value.to_string(); return }
        if self.items.len() >= self.max { self.items.pop_front(); }
        self.items.push_back(Memory { key: key.to_string(), value: value.to_string() });
    }
    pub fn recall(&self, key: &str) -> Option<&str> { self.items.iter().find(|m| m.key == key).map(|m| m.value.as_str()) }
    pub fn recent_context(&self, n: usize) -> String {
        self.items.iter().rev().take(n).map(|m| format!("{}:{}", m.key, m.value)).collect::<Vec<_>>().join("; ")
    }
}
