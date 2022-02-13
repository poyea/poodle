# Poodle [![Rust](https://github.com/poyea/poodle/actions/workflows/rust.yml/badge.svg)](https://github.com/poyea/poodle/actions/workflows/rust.yml) [![Release](https://github.com/poyea/poodle/actions/workflows/release.yml/badge.svg)](https://github.com/poyea/poodle/actions/workflows/release.yml) [![GitHub issues](https://img.shields.io/github/issues/poyea/poodle?color=red)](https://github.com/poyea/poodle/issues) [![GitHub forks](https://img.shields.io/github/forks/poyea/poodle)](https://github.com/poyea/poodle/network) [![GitHub stars](https://img.shields.io/github/stars/poyea/poodle?color=yellow)](https://github.com/poyea/poodle/stargazers) [![GitHub license](https://img.shields.io/github/license/poyea/poodle?color=white)](https://github.com/poyea/poodle/blob/main/LICENSE)
Not a 🐩 (Poodle), but a word-guessing game from your terminal. 🟩⬛🟩🟨🟩

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
<details>
    <summary>An example (SPOILER!!!)</summary>
    <img src="https://user-images.githubusercontent.com/24757020/153647249-ea80f3f6-f4fa-4593-9659-f6c92e5410cd.jpg" alt="Poodle screenshot">
</details>

#### Display full logs
```bash
$ poodle log
```

#### Get help message
```bash
$ poodle --help
```

## Building Poodle
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
This repository is licensed under MIT. See also [LICENSE](LICENSE) for details. Poodle is inspired by **Wordle**. **Wordle** is developed by Josh Wardle. The original game can be accessed [here](https://www.powerlanguage.co.uk/wordle/).
