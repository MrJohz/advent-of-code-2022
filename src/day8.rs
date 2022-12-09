pub struct Day8;

fn scenic_score(tree_map: &Vec<Vec<u8>>, (x, y): (usize, usize)) -> u32 {
    let my_size = tree_map[x][y];
    let mut scenic_score = 1;

    let mut sightline = 0;
    for x_prime in (0..x).rev() {
        sightline += 1;

        if tree_map[x_prime][y] >= my_size {
            break;
        }
    }
    scenic_score *= sightline;

    let mut sightline = 0;
    for x_prime in (x + 1)..tree_map.len() {
        sightline += 1;

        if tree_map[x_prime][y] >= my_size {
            break;
        }
    }
    scenic_score *= sightline;

    let mut sightline = 0;
    for y_prime in (0..y).rev() {
        sightline += 1;

        if tree_map[x][y_prime] >= my_size {
            break;
        }
    }
    scenic_score *= sightline;

    let mut sightline = 0;
    for y_prime in (y + 1)..tree_map.len() {
        sightline += 1;

        if tree_map[x][y_prime] >= my_size {
            break;
        }
    }
    scenic_score *= sightline;

    scenic_score
}

impl crate::runner::Day for Day8 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        let lines = input
            .lines()
            .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let length = lines.len();

        let mut visibility_map = vec![vec![false; length - 2]; length - 2];

        // horizontal sightlines
        for x in 1..(length - 1) {
            let mut minimum = lines[x][0];
            for y in 1..(length - 1) {
                if lines[x][y] > minimum {
                    minimum = lines[x][y];
                    visibility_map[x - 1][y - 1] = true;
                }

                if lines[x][y] == 9 {
                    break;
                }
            }

            let mut minimum = lines[x][length - 1];
            for y in (1..(length - 1)).rev() {
                if lines[x][y] > minimum {
                    minimum = lines[x][y];
                    visibility_map[x - 1][y - 1] = true;
                }

                if lines[x][y] == 9 {
                    break;
                }
            }
        }

        // vertical sightlines
        for y in 1..(length - 1) {
            let mut minimum = lines[0][y];
            for x in 1..(length - 1) {
                if lines[x][y] > minimum {
                    minimum = lines[x][y];
                    visibility_map[x - 1][y - 1] = true;
                }

                if lines[x][y] == 9 {
                    break;
                }
            }

            let mut minimum = lines[length - 1][y];
            for x in (1..(length - 1)).rev() {
                if lines[x][y] > minimum {
                    minimum = lines[x][y];
                    visibility_map[x - 1][y - 1] = true;
                }

                if lines[x][y] == 9 {
                    break;
                }
            }
        }

        let count = visibility_map.into_iter().flatten().filter(|b| *b).count() + length * 4 - 4;

        Ok(count.to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("1647")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let lines = input
            .lines()
            .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut scores = vec![vec![0; lines.len()]; lines.len()];

        for x in 1..(lines.len() - 1) {
            for y in 1..(lines.len() - 1) {
                scores[x][y] = scenic_score(&lines, (x, y))
            }
        }

        let best_score = scores
            .into_iter()
            .flatten()
            .fold(0, |acc, item| if item > acc { item } else { acc });

        Ok(best_score.to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("392080")
    }
}
