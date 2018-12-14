# Advent of Code

These are my Advent of Code solutions in Rust.

## Building

Runs (currently) on latest stable Rust. This project uses [cargo-aoc](https://github.com/gobanos/cargo-aoc). You can just `cargo run` the project, but using `cargo-aoc` allows you to only run a specific day.

To use `cargo-aoc`, first install it with `cargo install cargo-aoc`. Then you can run any day with `cargo aoc -d <day> -y <year> -p <part>`. If you omit some parameters, it defaults to running the current day, year or both parts. The benchmark can be run with `cargo aoc bench` with the same optional parameters.

## Benchmarks

These benchmarks were run on a Ryzen 7 1700X.

| Day |    Part 1 |    Part 2 | Notes                                                                                                              |
|----:|----------:|----------:|:-------------------------------------------------------------------------------------------------------------------|
|   1 | 140.54 ns | 287.16 µs |                                                                                                                    |
|   2 | 368.29 µs | 64.995 µs | Part 2 uses an inconsistent optimization by sorting the lines of IDs before comparison                             |
|   3 | 8.9421 ms | 8.0919 ms | Areas are stored in a quadtree                                                                                     |
|   4 | 95.937 µs | 96.591 µs |                                                                                                                    |
|   5 | 369.81 µs | 213.95 µs | Part 2 uses parallelization with rayon (don't ask why it's faster then part 1)                                     |
|   6 | 3.6438 ms | 856.39 µs | Too lazy to actually implement a proper Delaunay triangulation, so this one just got brute forced and parallelized |
|   7 | 11.206 µs | 11.372 µs | Topological sort with Kahn's algorithm                                                                             |
|   8 | 22.227 µs | 78.360 µs |                                                                                                                    |
|   9 | 482.00 µs |    ~58 ms |                                                                                                                    |
|  10 | 7.8872 ms | 6.0207 ms |                                                                                                                    |
|  11 | 1.0303 ms | 5.3645 ms |                                                                                                                    |
|  12 | 29.654 µs | 155.79 µs |                                                                                                                    |
|  13 | 106.07 µs | 2.0410 ms |                                                                                                                    |
|  14 | 3.2555 ms |   ~236 ms |                                                                                                                    |
