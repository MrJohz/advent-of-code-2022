pub struct Day4;

#[derive(Debug)]
struct Task {
    start: u32,
    end: u32,
}

fn parse_tasks(input: &str) -> (Task, Task) {
    let mut chars = input.chars().to_owned();

    let start_1 = (&mut chars)
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    let end_1 = (&mut chars)
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    let start_2 = (&mut chars)
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    let end_2 = (&mut chars)
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap();

    (
        Task {
            start: start_1,
            end: end_1,
        },
        Task {
            start: start_2,
            end: end_2,
        },
    )
}

fn is_contained(task1: Task, task2: Task) -> bool {
    (task1.start >= task2.start && task1.end <= task2.end)
        || (task2.start >= task1.start && task2.end <= task1.end)
}

fn has_overlap(task1: Task, task2: Task) -> bool {
    !((task1.start < task2.start && task1.end < task2.end)
        || (task2.start < task1.start && task2.end < task1.start))
}

impl crate::runner::Day for Day4 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .split('\n')
                .filter(|a| a.len() > 0)
                .map(parse_tasks)
                .map(|(task1, task2)| is_contained(task1, task2))
                .map(|each| each as u32)
                .sum::<u32>()
        ))
    }
    fn part_2(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .split('\n')
                .filter(|a| a.len() > 0)
                .map(parse_tasks)
                .map(|(task1, task2)| has_overlap(task1, task2))
                .map(|each| each as u32)
                .sum::<u32>()
        ))
    }
}
