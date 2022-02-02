#![allow(unused)]

mod cli;
mod data;
mod day;
mod logs;

use crate::day::DayState;

fn main() {
    let today = DayState::get_today();
    cli::exec(today);
}
