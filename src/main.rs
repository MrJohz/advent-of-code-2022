use clap::Parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod runner;

fn main() {
    let runner = runner::AdventOfCodeRunner::parse();
    runner.run().unwrap();
}
