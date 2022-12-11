use clap::Parser;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod runner;

fn main() {
    for _ in 0..25 {
        let runner = runner::AdventOfCodeRunner::parse();
        runner.run().unwrap();
    }
}
