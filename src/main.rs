#![allow(unused)]

mod cli;
mod logs;
mod state;
mod words;

fn main() {
    cli::exec(state::DayState::get_today());
}
