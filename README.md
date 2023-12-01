# Advent of Code - Rust

[![MIT License](https://badgen.net/badge/license/MIT/)](LICENSE.md)

My Rust solutions for [Advent of Code](https://adventofcode.com).

Heavenly inspired by [Tim Kurvers](https://github.com/timkurvers/advent-of-code/)

## Setup

Install [Rust] for your platform and make sure `cargo` is available.

## Authentication

The puzzle input will automatically downloaded in the `src/puzzle_inputs` folder.
But before that you need to first login to advent-of-code website and retrieve the session id from the cookie.

Then copy `.env.example` to `.env` and add your session id in the `AOC_SESSION` environment variable.

## Running solutions

To run a solution provide the year and day

```bash
cargo run 2015 1
```
or
```bash
cargo run 2015 01
```

## Development

To monitor code changes and re-run solutions during development, first:

```bash
cargo install cargo-watch
```

Then:

```bash
cargo watch -x "run 2015 01"
```

### Tests

To run tests for utilities:

```bash
cargo test
```

[Rust]: https://www.rust-lang.org/tools/install
