use std::collections::VecDeque;

pub struct Day11;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Square,
    Multiply(usize),
    Add(usize),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    inspect_count: usize,
    test_divisor: usize,
    test_true_id: usize,
    test_false_id: usize,
}

fn apply_operation(operation: Operation, item: usize) -> usize {
    match operation {
        Operation::Square => item * item,
        Operation::Multiply(n) => item * n,
        Operation::Add(n) => item + n,
    }
}

fn parse_operation(input: &str) -> Operation {
    match &input[0..1] {
        "+" => Operation::Add(input[2..].parse().unwrap()),
        "*" => {
            if &input[2..3] == "o" {
                Operation::Square
            } else {
                Operation::Multiply(input[2..].parse().unwrap())
            }
        }
        _ => panic!("Cannot understand input {:?}", input),
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines().filter(|each| !each.is_empty());
    let mut monkeys = Vec::new();

    while let (
        Some(_),
        Some(starting_items),
        Some(operation),
        Some(test_divisor),
        Some(test_true_id),
        Some(test_false_id),
    ) = (
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
        lines.next(),
    ) {
        let items = starting_items[18..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let operation = parse_operation(&operation[23..]);
        let test_divisor = test_divisor[21..].parse().unwrap();
        let test_true_id = test_true_id[29..].parse().unwrap();
        let test_false_id = test_false_id[30..].parse().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            inspect_count: 0,
            test_divisor,
            test_true_id,
            test_false_id,
        });
    }

    monkeys
}

fn process_monkeys(rounds: usize, monkeys: &mut [Monkey], on_worry: impl Fn(usize) -> usize) {
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                monkeys[idx].inspect_count += 1;
                let new_item = on_worry(apply_operation(monkeys[idx].operation, item));
                if new_item % monkeys[idx].test_divisor == 0 {
                    monkeys[monkeys[idx].test_true_id].items.push_back(new_item);
                } else {
                    monkeys[monkeys[idx].test_false_id]
                        .items
                        .push_back(new_item);
                }
            }
        }
    }
}

impl crate::runner::Day for Day11 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let mut monkeys = parse_monkeys(input);
        process_monkeys(
            20,
            &mut monkeys,
            #[inline(always)]
            |each| each / 3,
        );
        let (best, second_best) =
            monkeys
                .iter()
                .map(|m| m.inspect_count)
                .fold((0, 0), |(h1, h2), new| {
                    if new > h1 {
                        (new, h1)
                    } else if new > h2 {
                        (h1, new)
                    } else {
                        (h1, h2)
                    }
                });
        Ok((best * second_best).to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("90882")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let mut monkeys = parse_monkeys(input);
        let divisors = monkeys
            .iter()
            .map(|m| m.test_divisor)
            .fold(1, |prev, next| prev * next);

        process_monkeys(
            10_000,
            &mut monkeys,
            #[inline(always)]
            |each| each % divisors,
        );
        let (best, second_best) =
            monkeys
                .iter()
                .map(|m| m.inspect_count)
                .fold((0, 0), |(h1, h2), new| {
                    if new > h1 {
                        (new, h1)
                    } else if new > h2 {
                        (h1, new)
                    } else {
                        (h1, h2)
                    }
                });
        Ok((best * second_best).to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("30893109657")
    }
}
