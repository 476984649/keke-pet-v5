// 可可桌宠 v0.5 — 情绪引擎
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Mood { Happy, Sleepy, Grumpy, Lonely, Scared, Excited }

impl Mood {
    pub fn as_str(&self) -> &'static str {
        match self { Mood::Happy=>"happy",Mood::Sleepy=>"sleepy",Mood::Grumpy=>"grumpy",Mood::Lonely=>"lonely",Mood::Scared=>"scared",Mood::Excited=>"excited" }
    }
    pub fn emoji(&self) -> &'static str {
        match self { Mood::Happy=>"😊",Mood::Sleepy=>"😴",Mood::Grumpy=>"😤",Mood::Lonely=>"🥺",Mood::Scared=>"😰",Mood::Excited=>"🎉" }
    }
    pub fn compute(affection: u32, idle_secs: u64, hour: u32) -> Self {
        if hour>=22||hour<=5 { Mood::Sleepy }
        else if idle_secs>3600&&affection>50 { Mood::Lonely }
        else if affection<30 { Mood::Grumpy }
        else if affection>600 { Mood::Happy }
        else { Mood::Happy }
    }
}
