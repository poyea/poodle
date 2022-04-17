use crate::state::DayState;
use serde::{Deserialize, Serialize};
use std::{
    env, fmt,
    fs::{create_dir_all, read_to_string, File},
    path::Path,
};

static LOGS_TEMPLATE: &'static str = include_str!("./assets/logs.json");
static LOGS_LOCATION: &'static str = "/tmp/poodle/logs.json";

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
    fn get_logs() -> (Logs, String) {
        let location = match env::var("LOGS") {
            Ok(val) => val,
            Err(_) => LOGS_LOCATION.to_string(),
        };
        if Path::new(&location).is_file() {
            (
                serde_json::from_str(&read_to_string(&location).unwrap()).unwrap(),
                location,
            )
        } else {
            let path = Path::new(&location);
            let prefix = path.parent().unwrap();
            create_dir_all(prefix).unwrap();
            (serde_json::from_str(LOGS_TEMPLATE).unwrap(), location)
        }
    }

    pub fn save_log(state: DayState) {
        let (mut to_write_logs, location) = Logs::get_logs();
        to_write_logs.last_attempted = state.date.clone();
        to_write_logs.data.push(Pair {
            date: DayState::get_today(),
            dump: format!("{}", state),
        });
        serde_json::to_writer(&File::create(location).unwrap(), &to_write_logs).unwrap();
    }

    pub fn log() {
        println!("{}", Logs::get_logs().0)
    }
}

impl fmt::Display for Logs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for pair in &self.data {
            write!(f, "{}\n", pair.dump)?;
        }
        write!(f, "Last attempted: {}\n", self.last_attempted)?;
        Ok(())
    }
}
