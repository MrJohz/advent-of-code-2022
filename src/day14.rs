use std::{
    cmp,
    fmt::{Display, Write},
    ops::{Index, IndexMut},
    str::FromStr,
};

use itertools::Itertools;

pub struct Day14;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaveSliceElement {
    Sand,
    Rock,
    Air,
    Source,
}

#[derive(Debug)]
struct CaveSlice {
    x_start: usize,
    y_start: usize,

    cells: Vec<Vec<CaveSliceElement>>,
}

impl CaveSlice {
    fn get(&self, (x, y): (usize, usize)) -> Option<&CaveSliceElement> {
        let x = x - self.x_start;
        let y = y - self.y_start;
        if y >= self.cells.len() || x >= self.cells[0].len() {
            None
        } else {
            Some(&self.cells[y][x])
        }
    }

    fn insert_floor(&mut self) {
        self.cells
            .push(vec![CaveSliceElement::Air; self.cells[0].len()]);
        self.cells
            .push(vec![CaveSliceElement::Rock; self.cells[0].len()]);
    }

    /// Simulates the falling of a grain of sand from the source, until it can go no further
    ///
    /// Returns Some((x, y)) of the final location if the sand stops
    /// Returns None if the sand falls out of bounds
    fn simulate_sand(&mut self) -> Option<(usize, usize)> {
        let mut sand_point = (500, 0);
        if self[sand_point] != CaveSliceElement::Source {
            // cave is blocked, nothing can come in anyway
            return None;
        }

        'outer_loop: loop {
            for x_prime in [sand_point.0, sand_point.0 - 1, sand_point.0 + 1] {
                match self.get((x_prime, sand_point.1 + 1)) {
                    None => return None,
                    Some(CaveSliceElement::Air) => {
                        sand_point = (x_prime, sand_point.1 + 1);
                        continue 'outer_loop;
                    }
                    _ => {}
                }
            }

            // sand couldn't find anywhere better to go, so we've found
            // the end of the road
            self[sand_point] = CaveSliceElement::Sand;
            return Some(sand_point);
        }
    }
}

impl Index<(usize, usize)> for CaveSlice {
    type Output = CaveSliceElement;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y - self.y_start][x - self.x_start]
    }
}

impl IndexMut<(usize, usize)> for CaveSlice {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y - self.y_start][x - self.x_start]
    }
}

impl FromStr for CaveSlice {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_size = (500, 500);
        let mut y_size = (0, 0);
        let mut commands = Vec::new();

        for row in s.lines() {
            let mut current_commands = Vec::new();
            for pair in row.split(" -> ") {
                let (x, y) = pair
                    .split_once(',')
                    .ok_or(format!("could not split row {row:?}"))?;
                let x = x
                    .parse()
                    .map_err(|_| format!("could not parse string {x} as int"))?;
                let y = y
                    .parse()
                    .map_err(|_| format!("could not parse string {y} as int"))?;
                if x > x_size.1 {
                    x_size.1 = x;
                }
                if x < x_size.0 {
                    x_size.0 = x;
                }
                if y > y_size.1 {
                    y_size.1 = y;
                }
                if y < y_size.0 {
                    y_size.0 = y;
                }
                current_commands.push((x, y));
            }
            commands.push(current_commands);
        }

        let x_min_size = cmp::min(500 - (y_size.1 - y_size.0 + 1), x_size.0 + 1);
        let x_max_size = cmp::max(500 + (y_size.1 - y_size.0 + 1), x_size.1);

        let cells = vec![
            vec![CaveSliceElement::Air; x_max_size - x_min_size + 10];
            y_size.1 - y_size.0 + 1
        ];

        let mut cave_slice = CaveSlice {
            x_start: x_min_size - 5,
            y_start: y_size.0,
            cells,
        };

        cave_slice[(500, 0)] = CaveSliceElement::Source;

        for command in commands {
            for (start, finish) in command.into_iter().tuple_windows() {
                let x_range = if start.0 != finish.0 {
                    ((cmp::min(start.0, finish.0)), (cmp::max(start.0, finish.0)))
                } else {
                    (start.0, start.0)
                };
                let y_range = if start.1 != finish.1 {
                    ((cmp::min(start.1, finish.1)), (cmp::max(start.1, finish.1)))
                } else {
                    (start.1, start.1)
                };

                for x in x_range.0..=x_range.1 {
                    for y in y_range.0..=y_range.1 {
                        cave_slice[(x, y)] = CaveSliceElement::Rock
                    }
                }
            }
        }

        Ok(cave_slice)
    }
}

impl Display for CaveSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('.')?;
        for _ in 0..(self.cells[0].len()) {
            f.write_char('-')?;
        }
        f.write_char('.')?;
        f.write_char('\n')?;

        for row in &self.cells {
            f.write_char('|')?;
            for cell in row {
                f.write_char(match cell {
                    CaveSliceElement::Sand => 'o',
                    CaveSliceElement::Rock => '█',
                    CaveSliceElement::Air => ' ',
                    CaveSliceElement::Source => '+',
                })?;
            }
            f.write_char('|')?;
            f.write_char('\n')?;
        }
        f.write_char('\'')?;
        for _ in 0..(self.cells[0].len()) {
            f.write_char('-')?;
        }
        f.write_char('\'')?;

        Ok(())
    }
}

impl crate::runner::Day for Day14 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let mut slice = input.parse::<CaveSlice>().unwrap();
        let mut sand_counts = 0;
        while slice.simulate_sand().is_some() {
            sand_counts += 1;
        }

        Ok(sand_counts.to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("672")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let mut slice = input.parse::<CaveSlice>().unwrap();
        slice.insert_floor();
        let mut sand_counts = 0;
        while slice.simulate_sand().is_some() {
            sand_counts += 1;
        }

        Ok(sand_counts.to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("26831")
    }
}
