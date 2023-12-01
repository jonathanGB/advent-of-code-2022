# Advent Of Code 2022

[Link](https://adventofcode.com/2022)

## How To Run Solver

Make sure to be using the nightly compiler, as I use some unstable features. This can usually be done via `rustup default nightly`.

If you want to run the solver for day X, specifically part Y, run : `cargo run day[X] part[Y]`. For instance, to run the solver for day 5, specifically part 2, do `cargo run day5 part2`.

## Benchmarks

Some days have multiple solutions with benchmarks. To run them, you must be using a nightly Rust compiler (usually that's done with `rustup default nightly`), after which you can run the benchmarks with `cargo bench`. I have pasted benchmark results in the README of days that are available, which are:

* Day 6.