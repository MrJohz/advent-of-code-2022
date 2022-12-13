use std::cmp::Ordering;

use itertools::Itertools;
use rayon::slice::ParallelSliceMut;

pub struct Day13;

#[derive(Debug, Clone, PartialEq, Eq)]
struct DataStr<'a>(&'a [u8]);

impl<'a> DataStr<'a> {
    fn from_str(str: &'a str) -> Self {
        Self(str.as_bytes())
    }

    fn get_int(&'a self, idx: usize) -> (u8, usize) {
        match (self.0[idx], self.0[idx + 1]) {
            (b'1', b'0') => (10, idx + 2),
            (c, _) => (c - b'0', idx + 1),
        }
    }
}

impl<'a> PartialOrd for DataStr<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for DataStr<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut self_idx = 1;
        let mut self_nesting = 0;
        let mut other_idx = 1;
        let mut other_nesting = 0;

        loop {
            match (self.0[self_idx], other.0[other_idx]) {
                (b']', b']') => {
                    self_idx += 1;
                    other_idx += 1;
                }
                (b',' | b']', _) if self_nesting > 0 => {
                    self_nesting -= 1;
                    if self_nesting == 0 && self.0[self_idx] == b',' {
                        return Ordering::Less;
                    }
                }
                (_, b',' | b']') if other_nesting > 0 => {
                    other_nesting -= 1;
                    if other_nesting == 0 && other.0[other_idx] == b',' {
                        return Ordering::Greater;
                    }
                }

                (_, b']') => return Ordering::Greater,
                (b']', _) => return Ordering::Less,
                (b'0'..=b'9', b'0'..=b'9') => {
                    let (left, left_idx) = self.get_int(self_idx);
                    let (right, right_idx) = other.get_int(other_idx);
                    match left.cmp(&right) {
                        Ordering::Equal => {
                            self_idx = left_idx;
                            other_idx = right_idx;
                            continue;
                        }
                        ord => return ord,
                    }
                }
                (l, r) if l == r => {
                    self_idx += 1;
                    other_idx += 1;
                }
                (b'[', _) => {
                    self_idx += 1;
                    other_nesting += 1;
                }
                (_, b'[') => {
                    other_idx += 1;
                    self_nesting += 1;
                }
                (l, r) => panic!(
                    "Found {l} ({self_idx}) and {r} ({other_idx}) for {self:?} and {other:?}"
                ),
            }
        }
    }
}

impl crate::runner::Day for Day13 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let sum = input
            .lines()
            .filter(|l| !l.is_empty())
            .map(DataStr::from_str)
            .tuples()
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
        let divider_2 = DataStr(b"[[2]]");
        let divider_6 = DataStr(b"[[6]]");
        let mut items = vec![DataStr(b"[[2]]"), DataStr(b"[[6]]")];
        items.extend(
            input
                .lines()
                .filter(|l| !l.is_empty())
                .map(DataStr::from_str),
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
    use super::*;

    #[test]
    fn data_str_integers_works_as_expected() {
        assert!(DataStr(b"[1]") < DataStr(b"[2]"));
        assert!(DataStr(b"[2]") > DataStr(b"[1]"));
        assert!(DataStr(b"[10]") > DataStr(b"[9]"));
        assert!(DataStr(b"[1]") < DataStr(b"[10]"));
    }

    #[test]
    fn data_str_equal_sized_arrays_works_as_expected() {
        assert!(DataStr(b"[[1]]") < DataStr(b"[[2]]"));
        assert!(DataStr(b"[[2]]") > DataStr(b"[[1]]"));
    }

    #[test]
    fn data_str_unequally_equal_sized_arrays_works_as_expected() {
        assert!(DataStr(b"[[1,2]]") < DataStr(b"[[2]]"));
        assert!(DataStr(b"[[1]]") < DataStr(b"[[1,2]]"));
        assert!(DataStr(b"[[1,2]]") < DataStr(b"[[2,1]]"));
    }

    #[test]
    fn data_str_demo_example_1() {
        assert!(DataStr(b"[1,1,3,1,1]") < DataStr(b"[1,1,5,1,1]"),);
    }

    #[test]
    fn data_str_demo_example_2() {
        assert!(DataStr(b"[[1],[2,3,4]]") < DataStr(b"[[1],4]"),);
    }

    #[test]
    fn data_str_demo_example_3() {
        assert!(DataStr(b"[9]") > DataStr(b"[[8,7,6]]"),);
    }

    #[test]
    fn data_str_demo_example_4() {
        assert!(DataStr(b"[[4,4],4,4]") < DataStr(b"[[4,4],4,4,4]"),);
    }

    #[test]
    fn data_str_demo_example_5() {
        assert!(DataStr(b"[7,7,7,7]") > DataStr(b"[7,7,7]"),);
    }

    #[test]
    fn data_str_demo_example_6() {
        assert!(DataStr(b"[]") < DataStr(b"[3]"),);
    }

    #[test]
    fn data_str_demo_example_7() {
        assert!(DataStr(b"[[[]]]") > DataStr(b"[[]]"),);
    }

    #[test]
    fn data_str_buggy_case() {
        assert!(DataStr(b"[[[10,[6,6]]]]") > DataStr(b"[[10,[[9,10,0],2]]]]"));
    }
}
