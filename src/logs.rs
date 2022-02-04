use crate::state::DayState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::collections::HashMap;
use std::fmt;
use std::fs::{read_to_string, File};
use std::path::Path;

static LOGS: &'static str = include_str!("./assets/logs.json");
static LOGS_LOCATION: &'static str = "logs.json";

#[derive(Debug, Serialize, Deserialize)]
struct Pair {
    date: String,
    dump: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logs {
    data: Vec<Pair>,
    last_attempted: String,
}

impl Logs {
    pub fn get_logs() -> Logs {
        if Path::new(LOGS_LOCATION).is_file() {
            serde_json::from_str(&read_to_string(LOGS_LOCATION).unwrap()).unwrap()
        } else {
            serde_json::from_str(LOGS).unwrap()
        }
    }

    pub fn save_log(state: DayState) {
        let attempt_date = DayState::get_today();
        let mut to_write_logs = Logs::get_logs();
        to_write_logs.last_attempted = state.date.clone();
        to_write_logs.data.push(Pair {
            date: attempt_date,
            dump: format!("{}", state),
        });
        serde_json::to_writer(&File::create(LOGS_LOCATION).unwrap(), &to_write_logs).unwrap();
    }
}

impl fmt::Display for Logs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pair in &self.data {
            write!(f, "{}\n", pair.dump);
        }
        write!(f, "Last attempted: {}\n", self.last_attempted);
        Ok(())
    }
}
