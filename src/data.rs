use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::collections::HashMap;

static WORDS: &'static str = include_str!("./assets/words.json");

type Dict = HashMap<String, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Words {
    #[serde(flatten)]
    pub data: Dict,
}

pub fn get_words() -> Words {
    serde_json::from_str(WORDS).unwrap()
}
