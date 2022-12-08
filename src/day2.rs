pub struct Day2;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn score(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn opponents(self) -> (Self, Self, Self) {
        match self {
            Self::Rock => (Self::Paper, Self::Rock, Self::Scissors),
            Self::Paper => (Self::Scissors, Self::Paper, Self::Rock),
            Self::Scissors => (Self::Rock, Self::Scissors, Self::Paper),
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Unknown letter {}", c),
        }
    }
}

fn parse_line_1(input: &str) -> (Rps, Rps) {
    let mut ipt = input.chars();
    let first = Rps::from_char(ipt.next().unwrap());
    ipt.next();
    let second = Rps::from_char(ipt.next().unwrap());

    (first, second)
}

fn parse_line_2(input: &str) -> (Rps, Rps) {
    let mut ipt = input.chars();
    let first = Rps::from_char(ipt.next().unwrap());
    let opponents = first.opponents();
    ipt.next();
    let second = match ipt.next().unwrap() {
        'X' => opponents.2,
        'Y' => opponents.1,
        'Z' => opponents.0,
        c => panic!("Unknown letter {}", c),
    };

    (first, second)
}

fn score_match((left, right): (Rps, Rps)) -> u32 {
    let (_, draw, lose) = left.opponents();
    right.score()
        + if right == lose {
            0
        } else if right == draw {
            3
        } else {
            6
        }
}

impl crate::runner::Day for Day2 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .split('\n')
                .filter(|a| a.len() > 0)
                .map(parse_line_1)
                .map(score_match)
                .sum::<u32>()
        ))
    }
    fn part_2(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .split('\n')
                .filter(|a| a.len() > 0)
                .map(parse_line_2)
                .map(score_match)
                .sum::<u32>()
        ))
    }
}
