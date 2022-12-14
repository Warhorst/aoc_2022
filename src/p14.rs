use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use pad::{Position, PositionIter};
use pad::Direction::{Left, Right, Up};
use crate::input_reader::read_input;
use crate::p14::Field::*;
use crate::p14::SandUpdate::*;

pub fn solve_p14() {
    let input = read_input(14);
    let mut board = Board::from(input.as_str());

    let fallen_sand = board.run_sand();
    println!("Solution 1: {fallen_sand}");

    let mut board = Board::from(input.as_str());
    board.add_floor();
    let fallen_sand = board.run_sand();
    println!("Solution 2: {fallen_sand}");
}

struct Board {
    width: isize,
    height: isize,
    fields: HashMap<Position, Field>,
}

impl Board {
    fn run_sand(&mut self) -> usize {
        let mut sand_pos = self.sand_spawn();
        let mut fallen_sand = 0;

        loop {
            match self.get_next_position(sand_pos) {
                Fallen(new_pos) => sand_pos = new_pos,
                Calm => {
                    fallen_sand += 1;
                    self.fields.insert(sand_pos, Sand);

                    match self.fields.get(&self.sand_spawn()) {
                        Some(_) => return fallen_sand,
                        None => sand_pos = self.sand_spawn()
                    }
                }
                FallToBottom => return fallen_sand
            }
        }
    }

    fn sand_spawn(&self) -> Position {
        Position::new(500, 0)
    }

    fn get_next_position(&self, sand_pos: Position) -> SandUpdate {
        // A bit confusing. 'Up' means 'to the higher y pos'. Maybe I should rename this to something more general.
        let max_y = self.height - 1;
        let pos_down = sand_pos.position_in_direction(Up);

        if let None = self.fields.get(&pos_down) {
            return match pos_down.y == max_y {
                true => FallToBottom,
                false => Fallen(pos_down)
            };
        }

        let pos_down_left = sand_pos.position_in_direction(Up).position_in_direction(Left);

        if let None = self.fields.get(&pos_down_left) {
            return match pos_down_left.y == max_y {
                true => FallToBottom,
                false => Fallen(pos_down_left)
            };
        }

        let pos_down_right = sand_pos.position_in_direction(Up).position_in_direction(Right);

        if let None = self.fields.get(&pos_down_right) {
            return match pos_down_right.y == max_y {
                true => FallToBottom,
                false => Fallen(pos_down_right)
            };
        }

        Calm
    }

    fn add_floor(&mut self) {
        self.height += 2;
        let y = self.height - 1;
        Position::new(0, y).iter_to(Position::new(1000, y)).for_each(|pos| { self.fields.insert(pos, Rock); });
    }
}

enum SandUpdate {
    Fallen(Position),
    Calm,
    FallToBottom,
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let mut fields = HashMap::with_capacity(1000 * 200);

        input
            .lines()
            .map(Rocks::from)
            .for_each(|rock_positions| rock_positions.rocks.into_iter().for_each(|pos| { fields.insert(pos, Rock); }));

        let min_x = fields.keys().map(|pos| pos.x).min().unwrap();
        let max_x = fields.keys().map(|pos| pos.x).max().unwrap();
        let width = max_x - min_x + 1;
        let height = fields.keys().map(|pos| pos.y).max().unwrap() + 1;

        Board {
            width,
            height,
            fields,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        let min_x = self.fields.keys().map(|pos| pos.x).min().unwrap();

        for y in 0..self.height {
            for x in min_x..(min_x + self.width) {
                res.push_str(match self.fields.get(&Position::from((x, y))) {
                    Some(f) => match f {
                        Rock => "#",
                        Sand => "o",
                    },
                    None => "."
                })
            }

            res.push_str("\n");
        }

        write!(f, "{res}")
    }
}

enum Field {
    Rock,
    Sand,
}

struct Rocks {
    rocks: HashSet<Position>,
}

impl From<&str> for Rocks {
    fn from(line: &str) -> Self {
        let positions = line.split(" -> ")
            .map(|pos_str| {
                let mut split = pos_str.split(",");
                Position::new(
                    split.next().unwrap().parse::<isize>().unwrap(),
                    split.next().unwrap().parse::<isize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        Rocks {
            rocks: (0..positions.len() - 1)
                .flat_map(|i| create_positions_between(positions[i], positions[i + 1]))
                .collect()
        }
    }
}

fn create_positions_between(a: Position, b: Position) -> PositionIter {
    match a.x == b.x {
        false => match a.x < b.x {
            true => a.iter_to(b),
            false => b.iter_to(a)
        },
        true => match a.y < b.y {
            true => a.iter_to(b),
            false => b.iter_to(a)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p14::{Board, Rocks};

    #[test]
    fn rocks_from_line_works() {
        [
            "490,63 -> 495,63",
            "498,4 -> 498,6 -> 496,6"
        ].into_iter().for_each(|line| {
            let rocks = Rocks::from(line);
            println!("{:?}", rocks.rocks)
        })
    }

    #[test]
    fn examples_work() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let mut board = Board::from(input);
        assert_eq!(board.run_sand(), 24);

        let mut board = Board::from(input);
        board.add_floor();
        assert_eq!(board.run_sand(), 93);
    }
}
