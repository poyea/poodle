#![allow(unused)]

mod cli;
mod data;
mod day;
mod logs;

fn main() {
    cli::exec(day::DayState::get_today());
}
