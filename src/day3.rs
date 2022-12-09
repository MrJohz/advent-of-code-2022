use itertools::Itertools;

pub struct Day3;

fn split_at_center(input: &str) -> Vec<String> {
    vec![
        input[0..(input.len() / 2)].to_owned(),
        input[(input.len() / 2)..(input.len())].to_owned(),
    ]
}

fn find_common_element(strings: Vec<String>) -> u32 {
    let mut bits = [0_usize; 53];

    for (idx, string) in strings.iter().enumerate() {
        for c in string.chars() {
            let c = priority(c) as usize;
            if bits[c] == idx {
                bits[c] = idx + 1
            }
        }
    }

    for (index, &size) in bits.iter().enumerate() {
        if size == strings.len() {
            return index as u32;
        }
    }

    panic!("Found no elements common ");
}

fn priority(c: char) -> u32 {
    match c {
        c @ 'a'..='z' => (c) as u32 - 96,
        c @ 'A'..='Z' => (c) as u32 - 38,
        _ => panic!("character out of range"),
    }
}

impl crate::runner::Day for Day3 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .lines()
                .map(split_at_center)
                .map(find_common_element)
                .sum::<u32>()
        ))
    }
    fn part_2(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .lines()
                .chunks(3)
                .into_iter()
                .map(|c| c.map(|s| s.to_owned()).collect_vec())
                .map(find_common_element)
                .sum::<u32>()
        ))
    }
}
