use std::{
    fs,
    path::PathBuf,
    time::{self, Duration},
};

pub trait Day {
    fn part_1(_input: &str) -> anyhow::Result<String> {
        unimplemented!("part 1 of this day has not been implemented")
    }
    fn part_2(_input: &str) -> anyhow::Result<String> {
        unimplemented!("part 2 of this day has not been implemented")
    }

    fn expected_value_part_1() -> Option<&'static str> {
        None
    }

    fn expected_value_part_2() -> Option<&'static str> {
        None
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Input {
    Main,
    Demo,
}

impl Input {
    fn to_file_path(&self, day: u8) -> PathBuf {
        PathBuf::from("inputs").join(match self {
            Self::Main => format!("day_{}.txt", day),
            Self::Demo => format!("day_{}_demo.txt", day),
        })
    }
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq, Eq)]
enum Part {
    Both,
    Part1,
    Part2,
}

#[derive(clap::Parser, Debug)]
pub struct AdventOfCodeRunner {
    /// The day to execute
    #[arg(short, long)]
    day: Vec<u8>,

    /// The input to use
    #[arg(value_enum, short, long, default_value_t=Input::Main)]
    input: Input,

    /// The part to run
    #[arg(value_enum, short, long, default_value_t=Part::Both)]
    part: Part,
}

impl AdventOfCodeRunner {
    pub fn run(mut self) -> anyhow::Result<()> {
        if self.day.len() == 0 {
            self.day = vec![1, 2, 3, 4, 5, 6]
        }

        for day in &self.day {
            match day {
                1 => self.execute::<crate::day1::Day1>(*day)?,
                2 => self.execute::<crate::day2::Day2>(*day)?,
                3 => self.execute::<crate::day3::Day3>(*day)?,
                4 => self.execute::<crate::day4::Day4>(*day)?,
                5 => self.execute::<crate::day5::Day5>(*day)?,
                6 => self.execute::<crate::day6::Day6>(*day)?,
                n => panic!("Day not yet implemented: {}", n),
            };
        }

        Ok(())
    }

    fn execute<D: Day>(&self, day: u8) -> anyhow::Result<()> {
        let file_path = self.input.to_file_path(day);
        let input = fs::read_to_string(&file_path)?;

        println!("Executing day {}", day);
        println!("  [using input {:?}]", &file_path);
        println!();

        let mut duration = Duration::new(0, 0);

        if self.part == Part::Part1 || self.part == Part::Both {
            println!("Executing part 1");
            let (part_duration, output) = task_runner(&input, <D as Day>::part_1)?;
            println!("  {}", output);
            println!("  [duration = {:?}]", part_duration);
            if let Some(expected) = <D as Day>::expected_value_part_1() {
                println!(
                    "  [expected = {}{}]",
                    expected,
                    if expected == output { "" } else { " !FAILED!" }
                )
            }
            duration += part_duration;
        }

        if self.part == Part::Part2 || self.part == Part::Both {
            println!("Executing part 2");
            let (part_duration, output) = task_runner(&input, <D as Day>::part_2)?;
            println!("  {}", output);
            if let Some(expected) = <D as Day>::expected_value_part_2() {
                println!(
                    "  [expected = {}{}]",
                    expected,
                    if expected == output { "" } else { " !FAILED!" }
                )
            }
            println!("  [duration = {:?}]", part_duration);
            duration += part_duration;
        }

        println!();
        println!("Total duration: {:?}", duration);

        Ok(())
    }
}

fn task_runner(
    input: &str,
    f: impl Fn(&str) -> anyhow::Result<String>,
) -> anyhow::Result<(time::Duration, String)> {
    let response = f(input)?;

    let mut time_sum = Duration::default();

    for _ in 0..100 {
        let start = time::Instant::now();
        let res = f(input);
        let end = start.elapsed();
        time_sum += end;

        let res = res?;

        if res != response {
            panic!("Task is inconsistent - got {:?} then {:?}", response, res);
        }
    }

    time_sum /= 100;

    Ok((time_sum, response))
}
