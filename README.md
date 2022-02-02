# poodle [![Rust](https://github.com/poyea/poodle/actions/workflows/rust.yml/badge.svg)](https://github.com/poyea/poodle/actions/workflows/rust.yml)
Word-guessing game from your terminal 🟩🟨🟩🟨🟩.

## Features
* ✅ In Rust 🦀
* ✅ Attempt logs
* ✅ Rules of the [original game](https://www.powerlanguage.co.uk/wordle/)
* ✅ Customizable, extendable, localizable
* ✅ More to come

## How to use it

#### Guess today's riddle
```bash
$ poodle start
```

#### Display full logs
```bash
$ poodle log
```

#### Get help message
```bash
$ poodle -h
poodle 0.1.0
John Law <poyea@pm.me>
Word-guessing game from your terminal 🟩🟩🟩🟩🟩

USAGE:
    poodle.exe <CMD>

ARGS:
    <CMD>    Instruction [possible values: start, log]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```


## Building poodle
```sh
$ cargo -V
cargo 1.56.0 (4ed5d137b 2021-10-04)
$ cargo run                   # run
$ cargo build --all --release # build
$ cargo test --all --release  # test
$ cargo doc --all --release   # documentation
```

## If you like this, please
* Star
* Fork
* Contribute

## License
This repository is licensed under MIT. See also [LICENSE](LICENSE) for details. **Wordle** is developed by Josh Wardle. The original game can be accessed [here](https://www.powerlanguage.co.uk/wordle/).
