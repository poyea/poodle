mod cli;
mod keyboard;
mod logs;
mod state;
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
    let mut today_state = DayState::new(today_word);

    let stdin = io::stdin();
    'game: while today_state.remaining != 0 {
        {
            print!("{}", keyboard);
            print!("Your guess ({}) → ", today_state.remaining);
            io::stdout().flush().unwrap();
        }
        let mut buffer = String::new();
        {
            stdin.read_line(&mut buffer).unwrap();
            buffer = buffer.trim().to_string();
        }
        if DayState::input_hygiene(&buffer) {
            if DayState::input_allowed(&buffer, &allowed) {
                let attempt_fmt = today_state.guess(&buffer);
                keyboard.set(&today_state, &buffer);
                {
                    print!("\t\t{}", attempt_fmt);
                    io::stdout().flush().unwrap();
                }
                if today_state.finished() {
                    break 'game;
                }
            } else {
                println!("Unallowed word ← {}", buffer);
            }
        } else {
            println!("Invalid input  ← {}", buffer);
        }
    }
    {
        println!("{}", &today_state);
        Logs::save_log(today_state);
    }
}

fn print_clear_console() {
    print!("\x1B[2J")
}

pub fn exec(args: Cli) {
    let today = "Feb 02, 2022".to_string();
    print_clear_console();
    println!("[{}] Hello poodler!", &today);
    match args.cmd {
        Instruction::Start => start(today),
        Instruction::Log => Logs::log(),
    }
}
