use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::collections::HashMap;

static WORDS: &'static str = include_str!("./assets/words.json");
static ALLOWED: &'static str = include_str!("./assets/allowed.json");

#[derive(Debug, Serialize, Deserialize)]
pub struct Words {
    #[serde(flatten)]
    pub data: HashMap<String, String>,
}

pub fn get_words() -> Words {
    serde_json::from_str(WORDS).unwrap()
}

pub fn get_allowed() -> Vec<String> {
    serde_json::from_str(ALLOWED).unwrap()
}
