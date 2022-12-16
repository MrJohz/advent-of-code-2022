pub struct Day9;

enum Command {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

fn parse_command(input: &str) -> Command {
    let count = input[2..].parse().unwrap();
    match &input[0..1] {
        "U" => Command::Up(count),
        "D" => Command::Down(count),
        "L" => Command::Left(count),
        "R" => Command::Right(count),
        _ => panic!("unknown input {}", input),
    }
}

fn follow(prev_pos: &(isize, isize), curr_pos: &mut (isize, isize)) {
    if prev_pos == curr_pos {
        return;
    }

    if prev_pos.0 == curr_pos.0 {
        if prev_pos.1 > (curr_pos.1 + 1) {
            curr_pos.1 += 1;
        } else if prev_pos.1 < (curr_pos.1 - 1) {
            curr_pos.1 -= 1
        }
    } else if prev_pos.1 == curr_pos.1 {
        if prev_pos.0 > (curr_pos.0 + 1) {
            curr_pos.0 += 1;
        } else if prev_pos.0 < (curr_pos.0 - 1) {
            curr_pos.0 -= 1
        }
    }

    let x_diff = prev_pos.0 - curr_pos.0;
    let y_diff = prev_pos.1 - curr_pos.1;

    if x_diff > 1 || y_diff > 1 || x_diff < -1 || y_diff < -1 {
        curr_pos.0 += x_diff.clamp(-1, 1);
        curr_pos.1 += y_diff.clamp(-1, 1);
    }
}

fn follow_chain<const L: usize>(positions: &mut [(isize, isize); L]) {
    for i in 1..positions.len() {
        let prev_pos = positions[i - 1];
        let curr_pos = &mut positions[i];
        follow(&prev_pos, curr_pos);
    }
}

#[cfg(test)]
mod tests {
    use super::follow;

    #[test]
    fn test_follows_vertical_movement() {
        let mut curr_pos = (2, 0);

        follow(&(0, 0), &mut curr_pos);
        assert_eq!(curr_pos, (1, 0));

        follow(&(3, 0), &mut curr_pos);
        assert_eq!(curr_pos, (2, 0));
    }

    #[test]
    fn test_follows_horizontal_movement() {
        let mut curr_pos = (0, 2);

        follow(&(0, 0), &mut curr_pos);
        assert_eq!(curr_pos, (0, 1));

        follow(&(0, 3), &mut curr_pos);
        assert_eq!(curr_pos, (0, 2));
    }

    #[test]
    fn test_follows_diagonal_movement() {
        let mut curr_pos = (1, 2);

        follow(&(0, 0), &mut curr_pos);
        assert_eq!(curr_pos, (0, 1));

        follow(&(1, 3), &mut curr_pos);
        assert_eq!(curr_pos, (1, 2));

        follow(&(3, 3), &mut curr_pos);
        assert_eq!(curr_pos, (2, 3));

        follow(&(4, 2), &mut curr_pos);
        assert_eq!(curr_pos, (3, 2));

        follow(&(1, 0), &mut curr_pos);
        assert_eq!(curr_pos, (2, 1));
    }

    #[test]
    fn test_doesnt_move_if_items_are_equal() {
        let mut curr_pos = (1, 2);

        follow(&(1, 2), &mut curr_pos);
        assert_eq!(curr_pos, (1, 2));
    }

    #[test]
    fn test_doesnt_move_if_items_are_too_close() {
        let mut curr_pos = (1, 2);

        follow(&(1, 3), &mut curr_pos);
        assert_eq!(curr_pos, (1, 2));

        follow(&(2, 3), &mut curr_pos);
        assert_eq!(curr_pos, (1, 2));
    }
}

fn simulate<const L: usize>(
    commands: &[Command],
    seen_map: &mut SeenMap,
    positions: &mut [(isize, isize); L],
) {
    for command in commands {
        match command {
            Command::Up(n) => {
                for _ in 0..*n {
                    positions[0].1 += 1;
                    follow_chain(positions);
                    seen_map.mark_seen(*positions.last().unwrap());
                }
            }
            Command::Down(n) => {
                for _ in 0..*n {
                    positions[0].1 -= 1;
                    follow_chain(positions);
                    seen_map.mark_seen(*positions.last().unwrap());
                }
            }
            Command::Left(n) => {
                for _ in 0..*n {
                    positions[0].0 += 1;
                    follow_chain(positions);
                    seen_map.mark_seen(*positions.last().unwrap());
                }
            }
            Command::Right(n) => {
                for _ in 0..*n {
                    positions[0].0 -= 1;
                    follow_chain(positions);
                    seen_map.mark_seen(*positions.last().unwrap());
                }
            }
        }
    }
}

struct SeenMap {
    map: Vec<Vec<bool>>,
    start_pos: (isize, isize),
    seen: usize,
}

impl SeenMap {
    fn mark_seen(&mut self, (x, y): (isize, isize)) {
        let pos = &mut self.map[(x + self.start_pos.0) as usize][(y + self.start_pos.1) as usize];
        if !(*pos) {
            self.seen += 1;
        }
        *pos = true;
    }

    fn count_seen(&self) -> usize {
        self.seen
    }
}

fn build_seen_map(commands: &[Command]) -> SeenMap {
    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;
    let mut curr_pos = (0, 0);

    for command in commands {
        match command {
            Command::Up(n) => curr_pos.1 += n,
            Command::Down(n) => curr_pos.1 -= n,
            Command::Left(n) => curr_pos.0 += n,
            Command::Right(n) => curr_pos.0 -= n,
        }

        if curr_pos.1 > max_y {
            max_y = curr_pos.1;
        }
        if curr_pos.1 < min_y {
            min_y = curr_pos.1;
        }
        if curr_pos.0 > max_x {
            max_x = curr_pos.0;
        }
        if curr_pos.0 < min_x {
            min_x = curr_pos.0;
        }
    }

    SeenMap {
        map: vec![
            vec![false; (max_y - min_y + 2).try_into().unwrap()];
            (max_x - min_x + 2).try_into().unwrap()
        ],
        start_pos: (-min_x + 1, -min_y + 1),
        seen: 0,
    }
}

impl crate::runner::Day for Day9 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let mut positions = [(0, 0); 2];
        let commands = input.lines().map(parse_command).collect::<Vec<_>>();
        let mut seen_map = build_seen_map(&commands);
        simulate(&commands, &mut seen_map, &mut positions);
        Ok(seen_map.count_seen().to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("6236")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let mut positions = [(0, 0); 10];
        let commands = input.lines().map(parse_command).collect::<Vec<_>>();
        let mut seen_map = build_seen_map(&commands);
        simulate(&commands, &mut seen_map, &mut positions);
        Ok(seen_map.count_seen().to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("2449")
    }
}
