#![allow(unused)]

mod cli;
mod data;
mod logs;
mod state;

fn main() {
    cli::exec(state::DayState::get_today());
}
