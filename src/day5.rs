use itertools::Itertools;

pub struct Day5;

fn parse_structure<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<Vec<char>> {
    let lines = input.take_while(|line| !line.starts_with(" 1"));

    let mut v = Vec::new();

    for line in lines {
        if (line.len() + 1) / 4 != v.len() {
            v.resize_with((line.len() + 1) / 4, Vec::new)
        }

        for (idx, char) in line.chars().enumerate() {
            match char {
                'A'..='Z' => v[((idx + 3) / 4) - 1].push(char),
                _ => {}
            }
        }
    }

    for stack in &mut v {
        stack.reverse();
    }

    return v;
}

fn parse_commands<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<(u32, usize, usize)> {
    input
        .map(|line| {
            let two_digit = line.bytes().nth(6) != Some(' ' as u8);
            if two_digit {
                let count = line[5..7].parse().unwrap();
                let from = line[13..14].parse().unwrap();
                let to = line[18..19].parse().unwrap();

                (count, from, to)
            } else {
                let count = line[5..6].parse().unwrap();
                let from = line[12..13].parse().unwrap();
                let to = line[17..18].parse().unwrap();

                (count, from, to)
            }
        })
        .collect()
}

fn apply_actions_slowly(items: &mut [Vec<char>], commands: &[(u32, usize, usize)]) {
    for (count, from, to) in commands {
        for _ in 0..*count {
            if let Some(item) = items[from - 1].pop() {
                items[to - 1].push(item);
            }
        }
    }
}

fn apply_actions_quickly(items: &mut [Vec<char>], commands: &[(u32, usize, usize)]) {
    for (count, from, to) in commands {
        let input_len = items[from - 1].len();
        let mut stack = items[from - 1].split_off(input_len - (*count as usize));
        items[to - 1].append(&mut stack);
    }
}

impl crate::runner::Day for Day5 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let mut input = input.lines().filter(|l| !l.is_empty());
        let mut items = parse_structure(&mut input);
        let commands = parse_commands(&mut input);

        apply_actions_slowly(&mut items, &commands);

        Ok(items.iter().map(|v| v.last().unwrap()).join(""))
    }
    fn part_2(input: &str) -> anyhow::Result<String> {
        let mut input = input.lines().filter(|l| !l.is_empty());
        let mut items = parse_structure(&mut input);
        let commands = parse_commands(&mut input);

        apply_actions_quickly(&mut items, &commands);

        Ok(items.iter().map(|v| v.last().unwrap()).join(""))
    }
}
