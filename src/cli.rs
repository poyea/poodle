use clap::Parser;

/// Not a 🐩 (Poodle), but a word-guessing game from your terminal 🟩⬛🟩🟨🟩
#[derive(Debug, Parser)]
#[clap(author)]
#[clap(version)]
#[clap(long_about = None)]
pub struct Cli {
    /// Instruction
    #[clap(arg_enum)]
    pub cmd: Instruction,
}

#[derive(Debug, clap::ArgEnum, Clone)]
pub enum Instruction {
    Start,
    Log,
}
