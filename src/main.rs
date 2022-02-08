use clap::Parser;
use poodle::{exec, Cli};

fn main() {
    let args = Cli::parse();
    exec(args)
}
