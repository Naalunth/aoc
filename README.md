# Advent of Code

These are my Advent of Code solutions in Rust.

## Building

Runs (currently) on latest stable Rust. This project uses [cargo-aoc](https://github.com/gobanos/cargo-aoc). You can just `cargo run` the project, but using `cargo-aoc` allows you to only run a specific day.

To use `cargo-aoc`, first install it with `cargo install cargo-aoc`. Then you can run any day with `cargo aoc -d <day> -y <year> -p <part>`. If you omit some parameters, it defaults to running the current day, year or both parts. The benchmark can be run with `cargo aoc bench` with the same optional parameters.

## Benchmarks

These benchmarks were run on a Ryzen 7 1700X.

| Day | Part 1    | Part 2    | Notes                                                                                  |
|----:|----------:|----------:|:---------------------------------------------------------------------------------------|
| 1   | 140.54 ns | 287.16 µs |                                                                                        |
| 2   | 368.29 µs | 64.995 µs | Part 2 uses an inconsistent optimization by sorting the lines of IDs before comparison |
