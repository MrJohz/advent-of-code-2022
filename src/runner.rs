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
            self.day = vec![1, 2, 3, 4, 5]
        }

        for day in &self.day {
            match day {
                1 => self.execute::<crate::day1::Day1>(*day)?,
                2 => self.execute::<crate::day2::Day2>(*day)?,
                3 => self.execute::<crate::day3::Day3>(*day)?,
                4 => self.execute::<crate::day4::Day4>(*day)?,
                5 => self.execute::<crate::day5::Day5>(*day)?,
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
            let start = time::Instant::now();
            let output = <D as Day>::part_1(&input)?;
            let end = start.elapsed();
            println!("  {}", output);
            println!("  [duration = {:?}]", end);
            duration += end;
        }

        if self.part == Part::Part2 || self.part == Part::Both {
            println!("Executing part 2");
            let start = time::Instant::now();
            let output = <D as Day>::part_2(&input)?;
            let end = start.elapsed();
            println!("  {}", output);
            println!("  [duration = {:?}]", end);
            duration += end;
        }

        println!();
        println!("Total duration: {:?}", duration);

        Ok(())
    }
}
