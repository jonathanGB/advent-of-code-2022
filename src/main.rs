use clap::Parser;

mod day1;
mod args;
mod solver;

use args::{Args, Day};
use solver::Solver;
use day1::Day1Solver;

fn main() {
    let cli = Args::parse();

    match cli.day {
        Day::Day1 { part } => Day1Solver::solve(part),
    }
}
