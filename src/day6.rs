pub struct Day6;

fn unique_chars(text: &str) -> bool {
    let mut check_array = [false; 26];
    for b in text.as_bytes() {
        let prio = priority(*b);
        if check_array[prio] {
            return false;
        }
        check_array[prio] = true;
    }

    true
}

fn priority(c: u8) -> usize {
    match c as char {
        'a'..='z' => (c - 97).into(),
        _ => panic!("character out of range"),
    }
}

impl crate::runner::Day for Day6 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        for index in 0..input.len() {
            if unique_chars(&input[index..(index + 4)]) {
                return Ok(format!("{}", index + 4));
            }
        }

        panic!("No solution found");
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("1702")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        for index in 0..input.len() {
            if unique_chars(&input[index..(index + 14)]) {
                return Ok(format!("{}", index + 14));
            }
        }

        panic!("No solution found");
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("3559")
    }
}
