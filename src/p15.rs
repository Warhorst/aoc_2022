use pad::Direction::{Down, Left, Right, Up};
use pad::Position;
use crate::input_reader::read_input;

pub fn solve_p15() {
    let input = read_input(15);
    let board = Board::from(input.as_str());

    let count_occupied_positions_at_row = board.count_occupied_positions_at_row(2000000);
    println!("Solution 1: {count_occupied_positions_at_row}");

    let tuning_frequency = board.determine_tuning_frequency(4000000);
    println!("Solution 2: {tuning_frequency}");
}

struct Board {
    sensors_beacons: Vec<SensorBeacon>,
}

impl Board {
    fn count_occupied_positions_at_row(&self, row: isize) -> usize {
        let start = Position::new(self.min_x(), row);
        let end = Position::new(self.max_x(), row);

        start.iter_to(end)
            .into_iter()
            .filter(|pos| self.sensors_beacons.iter().all(|sb| sb.sensor != *pos && sb.beacon != *pos))
            .filter(|pos| self.sensors_beacons.iter().any(|sb| manhattan_distance(*pos, sb.sensor) <= sb.range))
            .count()
    }

    /// Disclaimer: I needed a hint from reddit to solve part 2. Thanks to
    /// https://www.reddit.com/r/adventofcode/comments/zmfwg1/2022_day_15_part_2_seekin_for_the_beacon/
    fn determine_tuning_frequency(&self, bound: isize) -> u128 {
        for i in 0..self.sensors_beacons.len() {
            let res = self.sensors_beacons[i]
                .perimeters()
                .into_iter()
                .filter(|pos| pos.x >= 0 && pos.x <= bound && pos.y >= 0 && pos.y <= bound)
                .filter(|pos| self.sensors_beacons
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .all(|(_, sb)| manhattan_distance(*pos, sb.sensor) > sb.range))
                .next();

            if let Some(res) = res {
                return res.x as u128 * 4000000 + res.y as u128
            }
        }

        panic!("no solution")
    }

    fn min_x(&self) -> isize {
        self.sensors_beacons
            .iter()
            .flat_map(|sb| [(sb, sb.sensor.x), (sb, sb.beacon.x)])
            .map(|(sb, x)| x - sb.range as isize)
            .min()
            .unwrap_or(0)
    }

    fn max_x(&self) -> isize {
        self.sensors_beacons
            .iter()
            .flat_map(|sb| [(sb, sb.sensor.x), (sb, sb.beacon.x)])
            .map(|(sb, x)| x + sb.range as isize)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Copy, Clone, Debug)]
struct SensorBeacon {
    sensor: Position,
    beacon: Position,
    range: usize,
}

impl SensorBeacon {
    fn perimeters<'a>(&'a self) -> impl IntoIterator<Item=Position> + 'a {
        let sensor_x = self.sensor.x;
        let sensor_y = self.sensor.y;
        let min_y = self.sensor.y - self.range as isize;
        let max_y = self.sensor.y + self.range as isize;

        (min_y..=max_y).into_iter()
            .flat_map(move |y| if y == min_y {
                let pos = Position::new(sensor_x, y);
                [
                    Some(pos.position_in_direction(Down)),
                    Some(pos.position_in_direction(Left)),
                    Some(pos.position_in_direction(Right))
                ]
            } else if y == max_y {
                let pos = Position::new(sensor_x, y);
                [
                    Some(pos.position_in_direction(Up)),
                    Some(pos.position_in_direction(Left)),
                    Some(pos.position_in_direction(Right))
                ]
            } else {
                let diff = (self.range - sensor_y.abs_diff(y)) as isize + 1;
                let left = Position::new(sensor_x - diff, y);
                let right = Position::new(sensor_x + diff, y);

                [Some(left), Some(right), None]
            })
            .flat_map(|opt| opt)
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        Board {
            sensors_beacons: input.lines().map(SensorBeacon::from).collect()
        }
    }
}

impl From<&str> for SensorBeacon {
    fn from(line: &str) -> Self {
        let cleaned = line.replace("Sensor at ", "")
            .replace(": closest beacon is at ", "|")
            .replace("x=", "")
            .replace(", y=", "|");
        let mut split = cleaned.split("|");

        let mut next_val = || split.next().unwrap().parse::<isize>().unwrap();

        let sensor = Position::new(next_val(), next_val());
        let beacon = Position::new(next_val(), next_val());

        SensorBeacon {
            sensor,
            beacon,
            range: manhattan_distance(sensor, beacon),
        }
    }
}

fn manhattan_distance(a: Position, b: Position) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

#[cfg(test)]
mod tests {
    use crate::p15::{Board, SensorBeacon};

    #[test]
    fn sensor_beacon_from_string_works() {
        let input = "Sensor at x=8, y=7: closest beacon is at x=2, y=10";
        let sb = SensorBeacon::from(input);
        println!("{:?}", sb);
    }

    #[test]
    fn examples_work() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        let board = Board::from(input);
        assert_eq!(board.count_occupied_positions_at_row(10), 26);
        assert_eq!(board.determine_tuning_frequency(20), 56000011);
    }

    #[test]
    fn get_perimeters_works() {
        let input = "Sensor at x=0, y=0: closest beacon is at x=2, y=2";
        let sb = SensorBeacon::from(input);

        sb.perimeters().into_iter().for_each(|pos| println!("{:?}", pos));
    }
}