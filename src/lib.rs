mod cli;
mod keyboard;
mod logs;
pub mod state;
mod words;

pub use cli::{Cli, Instruction};
pub use keyboard::Keyboard;
pub use logs::Logs;

use state::DayState;
use std::{io, io::Write};
use words::*;

fn start(today: String) {
    let ws = get_words();
    let allowed = get_allowed();

    let mut keyboard = Keyboard { keys: Vec::new() };
    keyboard.init();

    let today_word = ws.data[&today].to_string();
    let mut today_state = DayState::new(today_word, 6);

    let stdin = io::stdin();

    while !today_state.finished() {
        let ask_guess = || -> String {
            let mut buffer = String::new();
            print!("{}", keyboard);
            print!("Your guess ({}) → ", today_state.remaining_guess);
            io::stdout().flush().unwrap();
            stdin.read_line(&mut buffer).unwrap();
            buffer.trim().to_string()
        };
        let buffer = ask_guess();
        if !today_state.input_hygiene(&buffer) {
            println!("Invalid input  ← {}", buffer);
            continue;
        }
        if !today_state.input_allowed(&buffer, &allowed) {
            println!("Unallowed word ← {}", buffer);
            continue;
        }
        let attempt_fmt = today_state.guess(&buffer);
        keyboard.set_key_with_guess(&today_state, &buffer);

        print!("\t\t{}", attempt_fmt);
        io::stdout().flush().unwrap();
    }

    println!("{}", &today_state);
    Logs::save_log(today_state);
}

fn print_clear_console() {
    if cfg!(windows) {
        print!("\x1b[2J");
    } else if cfg!(unix) {
        print!("\x1B[2J");
    }
}

pub fn exec(args: Cli) {
    let today = state::DayState::get_today();
    print_clear_console();
    println!("[{}] Hello poodler!", &today);
    match args.cmd {
        Instruction::Start => start(today),
        Instruction::Log => Logs::log(),
    }
}
