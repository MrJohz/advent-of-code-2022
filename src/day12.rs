use std::{cmp::Reverse, collections::BinaryHeap, marker::PhantomData};

pub struct Day12;

trait Strategy {
    fn start_pos(height_map: &HeightMap<Self>) -> (usize, usize);
    fn can_step(height_map: &HeightMap<Self>, start: (usize, usize), end: (usize, usize)) -> bool;
    fn end_condition(height_map: &HeightMap<Self>, maybe_end: (usize, usize)) -> bool;
}

struct Part1Strategy;

impl Strategy for Part1Strategy {
    #[inline(always)]
    fn start_pos(height_map: &HeightMap<Self>) -> (usize, usize) {
        height_map.start_pos
    }
    #[inline(always)]
    fn can_step(height_map: &HeightMap<Self>, start: (usize, usize), end: (usize, usize)) -> bool {
        let current_height = height_map.heights[start.1][start.0];
        let future_height = height_map.heights[end.1][end.0];

        current_height + 1 >= future_height
    }
    #[inline(always)]
    fn end_condition(height_map: &HeightMap<Self>, maybe_end: (usize, usize)) -> bool {
        height_map.end_pos == maybe_end
    }
}

struct Part2Strategy;

impl Strategy for Part2Strategy {
    #[inline(always)]
    fn start_pos(height_map: &HeightMap<Self>) -> (usize, usize) {
        height_map.end_pos
    }
    #[inline(always)]
    fn can_step(height_map: &HeightMap<Self>, end: (usize, usize), start: (usize, usize)) -> bool {
        let current_height = height_map.heights[start.1][start.0];
        let future_height = height_map.heights[end.1][end.0];

        current_height + 1 >= future_height
    }
    #[inline(always)]
    fn end_condition(height_map: &HeightMap<Self>, maybe_end: (usize, usize)) -> bool {
        height_map.heights[maybe_end.1][maybe_end.0] == 0
    }
}

#[derive(Debug)]
struct HeightMap<T: Strategy + ?Sized> {
    heights: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    _strategy: PhantomData<T>,
}

impl<T: Strategy> HeightMap<T> {
    fn neighbours<'a>(
        &self,
        (x, y): (usize, usize),
        neighbour_space: &'a mut [(usize, usize); 4],
    ) -> &'a [(usize, usize)] {
        let mut idx = 0;

        if x > 0 && T::can_step(self, (x, y), (x - 1, y)) {
            neighbour_space[idx] = (x - 1, y);
            idx += 1;
        }

        if y > 0 && T::can_step(self, (x, y), (x, y - 1)) {
            neighbour_space[idx] = (x, y - 1);
            idx += 1;
        }

        if x < self.width - 1 && T::can_step(self, (x, y), (x + 1, y)) {
            neighbour_space[idx] = (x + 1, y);
            idx += 1;
        }

        if y < self.height - 1 && T::can_step(self, (x, y), (x, y + 1)) {
            neighbour_space[idx] = (x, y + 1);
            idx += 1;
        }

        &neighbour_space[0..idx]
    }

    fn steps_between(
        &self,
        start: (usize, usize),
        end_condition: impl Fn(&HeightMap<T>, (usize, usize)) -> bool,
    ) -> usize {
        let mut heap = BinaryHeap::new();
        let mut knowledge_map = vec![vec![(None, usize::MAX); self.width]; self.height];

        heap.push(Reverse((0, start)));
        knowledge_map[start.1][start.0].1 = 0;

        let mut neighbour_space = [(0, 0); 4];

        while let Some(elem) = heap.pop() {
            let (_, pos) = elem.0;
            if end_condition(self, pos) {
                let mut counts = 0;
                let mut pos = pos;
                while let Some(new_pos) = knowledge_map[pos.1][pos.0].0 {
                    counts += 1;
                    pos = new_pos;
                }
                return counts;
            }

            for &neighbour in self.neighbours(pos, &mut neighbour_space) {
                let potential_score = knowledge_map[pos.1][pos.0].1 + 1;
                if potential_score < knowledge_map[neighbour.1][neighbour.0].1 {
                    knowledge_map[neighbour.1][neighbour.0] = (Some(pos), potential_score);
                    heap.push(Reverse((potential_score, neighbour)));
                }
            }
        }

        panic!("Path not found!")
    }
}

fn parse_height_map<T: Strategy>(input: &str) -> HeightMap<T> {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    let heights = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .into_iter()
                .enumerate()
                .map(|(x, b)| match *b {
                    b'S' => {
                        start_pos = (x, y);
                        0
                    }
                    b'E' => {
                        end_pos = (x, y);
                        25
                    }
                    b => b - 97,
                })
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let height = heights.len();
    let width = heights[0].len();

    HeightMap {
        heights,
        height,
        width,
        start_pos,
        end_pos,
        _strategy: PhantomData,
    }
}

impl crate::runner::Day for Day12 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let height_map = parse_height_map::<Part1Strategy>(input);
        Ok(height_map
            .steps_between(height_map.start_pos, |hm, position| hm.end_pos == position)
            .to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("339")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let height_map = parse_height_map::<Part2Strategy>(input);
        Ok(height_map
            .steps_between(height_map.end_pos, |hm, position| {
                hm.heights[position.1][position.0] == 0
            })
            .to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("332")
    }
}
