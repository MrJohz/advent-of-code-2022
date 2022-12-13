use itertools::Itertools;
use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator},
    slice::ParallelSliceMut,
    str::ParallelString,
};

pub struct Day13;

#[derive(Debug, Clone, PartialEq, Eq, Ord, serde::Deserialize)]
#[serde(untagged)]
enum Data {
    Array(Vec<Data>),
    Integer(usize),
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(n1), Self::Integer(n2)) => n1.partial_cmp(n2),
            (Self::Array(a1), Self::Array(a2)) => a1.partial_cmp(a2),
            (Self::Array(a1), Self::Integer(n2)) => a1.partial_cmp(&vec![Self::Integer(*n2)]),
            (Self::Integer(n1), Self::Array(a2)) => vec![Self::Integer(*n1)].partial_cmp(a2),
        }
    }
}

impl crate::runner::Day for Day13 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let lines = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|s| s.as_bytes().to_vec())
            .tuples()
            .collect::<Vec<_>>();

        let sum = lines
            .into_par_iter()
            .map(|(mut s1, mut s2)| {
                (
                    simd_json::from_slice::<Data>(s1.as_mut()).unwrap(),
                    simd_json::from_slice::<Data>(s2.as_mut()).unwrap(),
                )
            })
            .enumerate()
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
        items.par_extend(
            input
                .par_lines()
                .filter(|l| !l.is_empty())
                .map(|s| s.as_bytes().to_vec())
                .map(|mut each| simd_json::from_slice::<Data>(each.as_mut()).unwrap()),
        );
        items.par_sort();

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
        assert_eq!(Integer(1) < Integer(2), true);
        assert_eq!(Integer(2) == Integer(2), true);
    }

    #[test]
    fn ordering_equal_sized_data_arrays_works_as_expected() {
        assert_eq!(Array(vec![Integer(1)]) < Array(vec![Integer(2)]), true);
        assert_eq!(Array(vec![Integer(2)]) == Array(vec![Integer(2)]), true);
    }

    #[test]
    fn ordering_nested_data_arrays_works_as_expected() {
        assert_eq!(
            Array(vec![Array(vec![Integer(1)])]) < Array(vec![Array(vec![Integer(2)])]),
            true
        );
        assert_eq!(
            Array(vec![Array(vec![Integer(2)])]) == Array(vec![Array(vec![Integer(2)])]),
            true
        );
    }

    #[test]
    fn ordering_unequally_sized_arrays_works_as_expected() {
        assert_eq!(
            Array(vec![Integer(1), Integer(2)]) < Array(vec![Integer(2)]),
            true
        );
        assert_eq!(
            Array(vec![Integer(1)]) < Array(vec![Integer(1), Integer(2)]),
            true
        );
        assert_eq!(
            Array(vec![Integer(1)]) < Array(vec![Integer(2), Integer(1)]),
            true
        );
    }

    #[test]
    fn demo_example_1() {
        assert_eq!(
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
            true
        );
    }

    #[test]
    fn demo_example_2() {
        assert_eq!(
            Array(vec![
                Array(vec![Integer(1)]),
                Array(vec![Integer(2), Integer(3), Integer(4)]),
            ]) < Array(vec![Array(vec![Integer(1)]), Integer(4)]),
            true
        );
    }

    #[test]
    fn demo_example_3() {
        assert_eq!(
            Array(vec![Integer(9)]) < Array(vec![Array(vec![Integer(8), Integer(7), Integer(6)])]),
            false
        );
    }

    #[test]
    fn demo_example_4() {
        assert_eq!(
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
            true
        );
    }

    #[test]
    fn demo_example_5() {
        assert_eq!(
            Array(vec![Integer(7), Integer(7), Integer(7), Integer(7)])
                < Array(vec![Integer(7), Integer(7), Integer(7)]),
            false
        );
    }

    #[test]
    fn demo_example_6() {
        assert_eq!(Array(vec![]) < Array(vec![Integer(3)]), true);
    }

    #[test]
    fn demo_example_7() {
        assert_eq!(
            Array(vec![Array(vec![Array(vec![])])]) < Array(vec![Array(vec![])]),
            false
        );
    }
}
