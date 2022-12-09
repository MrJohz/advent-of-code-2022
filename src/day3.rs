use itertools::Itertools;

pub struct Day3;

fn split_at_center(input: &str) -> Vec<String> {
    vec![
        input[0..(input.len() / 2)].to_owned(),
        input[(input.len() / 2)..(input.len())].to_owned(),
    ]
}

fn find_common_element(strings: Vec<String>) -> u32 {
    let mut bits_vec = Vec::new();

    for string in &strings {
        let mut bits_in_string = [0_usize; 53];
        for c in string.chars() {
            bits_in_string[priority(c) as usize] = 1;
        }

        bits_vec.push(bits_in_string);
    }

    let mut bits = [0_usize; 53];

    for bits_in_string in bits_vec {
        for index in 0..bits.len() {
            bits[index] += bits_in_string[index];
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
                .split('\n')
                .filter(|a| a.len() > 0)
                .map(split_at_center)
                .map(find_common_element)
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
                .sum::<u32>()
        ))
    }
}
