#![allow(unused)]

mod cli;
mod words;
mod logs;
mod state;

fn main() {
    cli::exec(state::DayState::get_today());
}
