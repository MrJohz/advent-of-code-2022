use clap::Parser;

mod day1;
mod day2;
mod runner;

fn main() {
    let runner = runner::AdventOfCodeRunner::parse();
    runner.run().unwrap();
}
