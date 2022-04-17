# Contribution Guidelines

We are under development! Feel free to make our Contribution Guidelines better!

#### Setting up Poodle
```sh
$ cargo -V
cargo 1.56.0 (4ed5d137b 2021-10-04)
$ cargo install poodle        # install from crates.io! (Recommended)
$ cargo install --path .      # or install locally
$ cargo run -- start          # or run directly
```

## Building Poodle for development
```sh
$ cargo build --all --release # build
$ cargo test --all --release  # test
$ cargo doc --all --release   # documentation
```