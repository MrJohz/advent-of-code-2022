use std::cmp;

pub struct Day15;

fn parse(input: &str) -> impl Iterator<Item = ((i64, i64), (i64, i64))> + '_ {
    input.lines().map(|input| {
        let input = input.as_bytes();
        let mut parts = [0; 4];
        let mut start_index = 12;
        let mut end_index;

        for part in &mut parts {
            loop {
                match input.get(start_index) {
                    Some(b'0'..=b'9') => break,
                    _ => start_index += 1,
                }
            }

            end_index = start_index;
            while let Some(b'0'..=b'9') = input.get(end_index) {
                end_index += 1;
            }

            *part = std::str::from_utf8(&input[start_index..end_index])
                .unwrap()
                .parse()
                .unwrap();

            start_index = end_index;
        }
        ((parts[0], parts[1]), (parts[2], parts[3]))
    })
}

fn manhatten(point1: (i64, i64), point2: (i64, i64)) -> i64 {
    (point1.0 - point2.0).abs() + (point1.1 - point2.1).abs()
}

impl crate::runner::Day for Day15 {
    fn part_1(input: &str) -> anyhow::Result<String> {
        const Y_LEVEL: i64 = 2000000; //10;
        let mut smallest = i64::MAX;
        let mut largest = i64::MIN;
        let (mut sensors, beacons): (Vec<_>, Vec<_>) = parse(input)
            .map(|(sensor, beacon)| ((sensor, manhatten(sensor, beacon)), beacon))
            .inspect(|((sensor, manhatten), _)| {
                // we can definitely make this cleverer by reducing the manhatten distance down
                // but let's save that for later if we need it
                smallest = cmp::min(smallest, sensor.0 - manhatten + (Y_LEVEL - sensor.1).abs());
                largest = cmp::max(largest, sensor.0 + manhatten - (Y_LEVEL - sensor.1).abs());
            })
            .unzip();
        sensors.sort_unstable_by_key(|(_, distance)| -distance);

        let mut found_count = 0;
        let mut idx = smallest;
        'outer: while idx <= largest {
            for beacon in &beacons {
                if *beacon == (idx, Y_LEVEL) {
                    idx += 1;
                    continue 'outer;
                }
            }

            for (sensor, distance_to_beacon) in &sensors {
                if manhatten((idx, Y_LEVEL), *sensor) <= *distance_to_beacon {
                    let y_distance_to_sensor = (sensor.1 - Y_LEVEL).abs();
                    let x_distance_to_sensor = sensor.0 - idx;
                    let visible_along_x_axis = distance_to_beacon - y_distance_to_sensor;
                    let skippable = cmp::max(x_distance_to_sensor + visible_along_x_axis, 1);
                    idx += skippable;
                    found_count += skippable;
                    continue 'outer;
                }
            }

            idx += 1;
        }

        Ok(found_count.to_string())
    }
    fn expected_value_part_1() -> Option<&'static str> {
        Some("5809294")
    }

    fn part_2(input: &str) -> anyhow::Result<String> {
        let (mut sensors, beacons): (Vec<_>, Vec<_>) = parse(input)
            .map(|(sensor, beacon)| ((sensor, manhatten(sensor, beacon)), beacon))
            .unzip();
        sensors.sort_unstable_by_key(|(_, distance)| -distance);

        let mut found_position = None;
        'y_loop: for y in 0..=4_000_000 {
            let mut idx = 0;
            'x_loop: while idx <= 4_000_000 {
                for beacon in &beacons {
                    if *beacon == (idx, y) {
                        idx += 1;
                        continue 'x_loop;
                    }
                }

                for (sensor, distance_to_beacon) in &sensors {
                    if manhatten((idx, y), *sensor) <= *distance_to_beacon {
                        let y_distance_to_sensor = (sensor.1 - y).abs();
                        let x_distance_to_sensor = sensor.0 - idx;
                        let visible_along_x_axis = distance_to_beacon - y_distance_to_sensor;
                        let skippable = cmp::max(x_distance_to_sensor + visible_along_x_axis, 1);
                        idx += skippable;
                        continue 'x_loop;
                    }
                }

                found_position = Some((idx, y));
                break 'y_loop;
            }
        }

        let found_position = found_position.unwrap();

        Ok((found_position.0 * 4_000_000 + found_position.1).to_string())
    }
    fn expected_value_part_2() -> Option<&'static str> {
        Some("10693731308112")
    }
}
