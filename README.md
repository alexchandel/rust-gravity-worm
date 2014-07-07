rust-gravity-worm ![Build Status](https://travis-ci.org/alexchandel/rust-gravity-worm.png)
=================

A textual Gravity Worm game written in Rust using the ncurses TUI library.

## Deps
* [ncurses-rs](https://github.com/jeaye/ncurses-rs) — the Rust-ncurses bindings library.

## Setup
You need a few things to run rust-gravity-worm:

1.	[Rust's master branch](https://github.com/rust-lang/rust)
2.	A terminal/console/command-prompt with ncurses.
3.	Rust's [cargo](https://github.com/rust-lang/cargo) manager

If you don't want to use cargo, you will need to clone, compile, and link `ncurses-rs` yourself.

## Building
To build with `cargo`, `cd` to the repository's root and run:
```bash
cargo-build
```

## Running
The binary will be placed in the `target` directory by `cargo`. It can be run with:
```bash
target/worm
```
