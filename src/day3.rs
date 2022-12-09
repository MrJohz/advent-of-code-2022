use std::collections::HashSet;

use itertools::Itertools;

pub struct Day3;

fn split_at_center(input: &str) -> Vec<String> {
    vec![
        input[0..(input.len() / 2)].to_owned(),
        input[(input.len() / 2)..(input.len())].to_owned(),
    ]
}

fn find_common_element(strings: Vec<String>) -> char {
    let sets = strings
        .iter()
        .skip(1)
        .map(|s| HashSet::<_>::from_iter(s.chars()))
        .collect::<Vec<_>>();

    'outer: for elem in strings.first().unwrap().chars() {
        for set in &sets {
            if !set.contains(&elem) {
                continue 'outer;
            }
        }

        return elem;
    }

    panic!("Found no elements common ")
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
                .split('\n')
                .filter(|a| a.len() > 0)
                .map(split_at_center)
                .map(find_common_element)
                .map(priority)
                .sum::<u32>()
        ))
    }
    fn part_2(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .split('\n')
                .filter(|a| a.len() > 0)
                .chunks(3)
                .into_iter()
                .map(|c| c.map(|s| s.to_owned()).collect_vec())
                .map(find_common_element)
                .map(priority)
                .sum::<u32>()
        ))
    }
}
