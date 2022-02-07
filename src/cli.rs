use crate::{logs::*, state::*, words::*};
use clap::Parser;
use std::{io, io::Write};

/// Not a 🐩 (Poodle), but a word-guessing game from your terminal 🟩⬛🟩🟨🟩
#[derive(Debug, Parser)]
#[clap(author)]
#[clap(version)]
#[clap(long_about = None)]
pub struct Cli {
    /// Instruction
    #[clap(arg_enum)]
    cmd: Instruction,
}

#[derive(Debug, clap::ArgEnum, Clone)]
pub enum Instruction {
    Start,
    Log,
}

fn start(today: String) {
    let ws = get_words();
    let allowed = get_allowed();

    let today_word = ws.data[&today].to_string();
    let mut today_state = DayState::new(today_word);

    let mut stdin = io::stdin();
    'game: while today_state.remaining != 0 {
        {
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
                let attempt_fmt = today_state.guess(buffer);
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

fn log() {
    let logs = Logs::get_logs();
    println!("{}", logs);
}

pub fn exec(today: String) {
    let args = Cli::parse();
    println!("[{}] Hello poodler!", &today);
    match args.cmd {
        Instruction::Start => start(today),
        Instruction::Log => log(),
    }
}
