pub struct Day4;

#[derive(Debug, Default)]
struct Task {
    start: u32,
    end: u32,
}

fn parse_tasks(input: &str) -> (Task, Task) {
    let mut tasks = <(Task, Task)>::default();
    let mut task_strings = input.split(',');

    let mut task_1_str = task_strings.next().unwrap().split('-');
    tasks.0.start = task_1_str.next().unwrap().parse().unwrap();
    tasks.0.end = task_1_str.next().unwrap().parse().unwrap();

    let mut task_2_str = task_strings.next().unwrap().split('-');
    tasks.1.start = task_2_str.next().unwrap().parse().unwrap();
    tasks.1.end = task_2_str.next().unwrap().parse().unwrap();

    tasks
}

fn is_contained(task1: Task, task2: Task) -> bool {
    (task1.start >= task2.start && task1.end <= task2.end)
        || (task2.start >= task1.start && task2.end <= task1.end)
}

fn has_overlap(task1: Task, task2: Task) -> bool {
    !((task1.start < task2.start && task1.end < task2.start)
        || (task2.start < task1.start && task2.end < task1.start))
}

impl crate::runner::Day for Day4 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .lines()
                .map(parse_tasks)
                .map(|(task1, task2)| is_contained(task1, task2))
                .map(|each| each as u32)
                .sum::<u32>()
        ))
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("441")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            input
                .lines()
                .map(parse_tasks)
                .map(|(task1, task2)| has_overlap(task1, task2))
                .map(|each| each as u32)
                .sum::<u32>()
        ))
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("861")
    }
}
