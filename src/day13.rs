use std::str::FromStr;

use itertools::Itertools;
use rayon::{
    prelude::{ParallelBridge, ParallelIterator},
    slice::ParallelSliceMut,
};

pub struct Day13;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(untagged)]
enum Data {
    Array(Vec<Data>),
    Integer(usize),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Integer(n1), Self::Integer(n2)) => n1.cmp(n2),
            (Self::Array(a1), Self::Array(a2)) => a1.cmp(a2),
            (Self::Array(a1), Self::Integer(n2)) => a1.cmp(&vec![Self::Integer(*n2)]),
            (Self::Integer(n1), Self::Array(a2)) => vec![Self::Integer(*n1)].cmp(a2),
        }
    }
}

impl Data {
    fn create_int_from_bytes(start_idx: usize, bytes: &[u8]) -> (Self, usize) {
        match (bytes[start_idx], bytes[start_idx + 1]) {
            (b'1', b'0') => (Self::Integer(10), start_idx + 2),
            (c, _) => (Self::Integer((c - b'0') as usize), start_idx + 1),
        }
    }

    fn create_array_from_bytes(mut start_idx: usize, bytes: &[u8]) -> (Self, usize) {
        start_idx += 1;
        let mut elems = Vec::new();

        loop {
            match bytes[start_idx] {
                b']' => return (Self::Array(elems), start_idx + 1),
                b'0'..=b'9' => {
                    let (elem, idx) = Self::create_int_from_bytes(start_idx, bytes);
                    elems.push(elem);
                    start_idx = idx;
                    continue;
                }
                b'[' => {
                    let (elem, idx) = Self::create_array_from_bytes(start_idx, bytes);
                    elems.push(elem);
                    start_idx = idx;
                    continue;
                }
                _ => start_idx += 1,
            }
        }
    }
}

impl FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = &s.as_bytes();

        let (me, _) = Self::create_array_from_bytes(0, bytes);

        Ok(me)
    }
}

impl crate::runner::Day for Day13 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let sum = input
            .lines()
            .filter(|l| !l.is_empty())
            .tuples()
            .enumerate()
            .par_bridge()
            .map(|(idx, (s1, s2))| {
                (
                    idx,
                    (s1.parse::<Data>().unwrap(), s2.parse::<Data>().unwrap()),
                )
            })
            .filter(|(_, (left, right))| left < right)
            .map(|(idx, _)| idx + 1)
            .sum::<usize>();
        Ok(sum.to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("5675")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let divider_2 = Data::Array(vec![Data::Array(vec![Data::Integer(2)])]);
        let divider_6 = Data::Array(vec![Data::Array(vec![Data::Integer(6)])]);
        let mut items = vec![divider_2.clone(), divider_6.clone()];
        items.extend(
            input
                .lines()
                .filter(|l| !l.is_empty())
                .map(|each| each.parse().unwrap()),
        );
        items.par_sort_unstable();

        let sum = items
            .into_iter()
            .enumerate()
            .filter(|(_, item)| item == &divider_2 || item == &divider_6)
            .map(|(idx, _)| idx + 1)
            .product::<usize>();

        Ok(sum.to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("20383")
    }
}

#[cfg(test)]
mod tests {
    use super::Data::*;

    #[test]
    fn ordering_data_integers_works_as_expected() {
        assert!(Integer(1) < Integer(2));
        assert!(Integer(2) == Integer(2));
    }

    #[test]
    fn ordering_equal_sized_data_arrays_works_as_expected() {
        assert!(Array(vec![Integer(1)]) < Array(vec![Integer(2)]));
        assert!(Array(vec![Integer(2)]) == Array(vec![Integer(2)]));
    }

    #[test]
    fn ordering_nested_data_arrays_works_as_expected() {
        assert!(Array(vec![Array(vec![Integer(1)])]) < Array(vec![Array(vec![Integer(2)])]));
        assert!(Array(vec![Array(vec![Integer(2)])]) == Array(vec![Array(vec![Integer(2)])]));
    }

    #[test]
    fn ordering_unequally_sized_arrays_works_as_expected() {
        assert!(Array(vec![Integer(1), Integer(2)]) < Array(vec![Integer(2)]));
        assert!(Array(vec![Integer(1)]) < Array(vec![Integer(1), Integer(2)]));
        assert!(Array(vec![Integer(1)]) < Array(vec![Integer(2), Integer(1)]));
    }

    #[test]
    fn demo_example_1() {
        assert!(
            Array(vec![
                Integer(1),
                Integer(1),
                Integer(3),
                Integer(1),
                Integer(1)
            ]) < Array(vec![
                Integer(1),
                Integer(1),
                Integer(5),
                Integer(1),
                Integer(1)
            ]),
        );
    }

    #[test]
    fn demo_example_2() {
        assert!(
            Array(vec![
                Array(vec![Integer(1)]),
                Array(vec![Integer(2), Integer(3), Integer(4)]),
            ]) < Array(vec![Array(vec![Integer(1)]), Integer(4)]),
        );
    }

    #[test]
    fn demo_example_3() {
        assert!(
            Array(vec![Integer(9)]) > Array(vec![Array(vec![Integer(8), Integer(7), Integer(6)])])
        );
    }

    #[test]
    fn demo_example_4() {
        assert!(
            Array(vec![
                Array(vec![Integer(4), Integer(4)]),
                Integer(4),
                Integer(4)
            ]) < Array(vec![
                Array(vec![Integer(4), Integer(4)]),
                Integer(4),
                Integer(4),
                Integer(4)
            ]),
        );
    }

    #[test]
    fn demo_example_5() {
        assert!(
            Array(vec![Integer(7), Integer(7), Integer(7), Integer(7)])
                > Array(vec![Integer(7), Integer(7), Integer(7)])
        );
    }

    #[test]
    fn demo_example_6() {
        assert!(Array(vec![]) < Array(vec![Integer(3)]));
    }

    #[test]
    fn demo_example_7() {
        assert!(Array(vec![Array(vec![Array(vec![])])]) > Array(vec![Array(vec![])]));
    }

    #[test]
    fn parses_array_of_numbers() {
        assert_eq!(
            Array(vec![Integer(1), Integer(2), Integer(10), Integer(0)]),
            "[1,2,10,0]".parse().unwrap()
        )
    }

    #[test]
    fn parses_empty_array() {
        assert_eq!(Array(vec![]), "[]".parse().unwrap())
    }

    #[test]
    fn parses_nested_arrays() {
        assert_eq!(
            Array(vec![
                Integer(1),
                Array(vec![]),
                Array(vec![Integer(1)]),
                Array(vec![Array(vec![Integer(1)])])
            ]),
            "[1,[],[1],[[1]]]".parse().unwrap()
        )
    }
}
