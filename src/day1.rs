pub struct Day1;

fn find_weight_totals(input: &str) -> anyhow::Result<Vec<u32>> {
    let mut weights = vec![0];

    for line in input.split('\n') {
        if line == "" {
            weights.push(0);
            continue;
        }

        let last_index = weights.len() - 1;
        weights[last_index] += line.parse::<u32>()?;
    }

    Ok(weights)
}

fn find_largest(items: &[u32]) -> u32 {
    items
        .iter()
        .fold(0, |prev, &curr| if curr > prev { curr } else { prev })
}

fn find_largest_three(items: &[u32]) -> (u32, u32, u32) {
    items.iter().fold((0, 0, 0), |(p1, p2, p3), &curr| {
        if curr > p1 {
            (curr, p1, p2)
        } else if curr > p2 {
            (p1, curr, p2)
        } else if curr > p3 {
            (p1, p2, curr)
        } else {
            (p1, p2, p3)
        }
    })
}

impl crate::runner::Day for Day1 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let weights = find_weight_totals(input)?;
        Ok(format!("{}", find_largest(&weights)))
    }

    fn expected_value_part_1() -> Option<&'static str> {
        Some("71924")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let weights = find_weight_totals(input)?;
        let (w1, w2, w3) = find_largest_three(&weights);
        Ok(format!("{}", w1 + w2 + w3))
    }

    fn expected_value_part_2() -> Option<&'static str> {
        Some("210406")
    }
}
