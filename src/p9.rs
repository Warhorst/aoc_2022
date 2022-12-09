use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::Formatter;
use crate::input_reader::read_input;
use Direction::*;

pub fn solve_p9() {
    let input = read_input(9);
    let mut board = Board::new();
    board.process_all_steps(input.lines().map(Steps::from));

    let num_visited_tail_positions = board.num_tail_visited_positions();
    print!("Solution 1: {num_visited_tail_positions}");

    let mut board_ext = BoardExtended::new();
    board_ext.process_all_steps(input.lines().map(Steps::from));

    let num_visited_tail_positions = board_ext.num_tail_visited_positions();
    print!("Solution 2: {num_visited_tail_positions}");
}

struct Board {
    head_position: Position,
    tail_position: Position,
    tail_visited_positions: HashSet<Position>,
}

impl Board {
    fn new() -> Self {
        Board {
            head_position: Position::new(0, 0),
            tail_position: Position::new(0, 0),
            tail_visited_positions: [Position::new(0, 0)].into_iter().collect(),
        }
    }

    fn process_all_steps(&mut self, steps: impl IntoIterator<Item=Steps>) {
        steps.into_iter().for_each(|steps| self.process_steps(steps))
    }

    fn process_steps(&mut self, steps: Steps) {
        let dir = steps.direction;

        for _ in 0..steps.amount {
            self.head_position = self.head_position.position_in_direction(dir);

            if !self.tail_position.neighboured_with(&self.head_position) {
                self.tail_position = match dir {
                    Left | Right => Position::new(self.tail_position.x, self.head_position.y).position_in_direction(dir),
                    Up | Down => Position::new(self.head_position.x, self.tail_position.y).position_in_direction(dir),
                };
                self.tail_visited_positions.insert(self.tail_position);
            }
        }
    }

    fn num_tail_visited_positions(&self) -> usize {
        self.tail_visited_positions.len()
    }
}

struct BoardExtended {
    rope: Vec<Position>,
    tail_visited_positions: HashSet<Position>,
}

impl BoardExtended {
    fn new() -> Self {
        BoardExtended {
            rope: vec![Position::new(0, 0); 10],
            tail_visited_positions: [Position::new(0, 0)].into_iter().collect(),
        }
    }

    fn process_all_steps(&mut self, steps: impl IntoIterator<Item=Steps>) {
        steps.into_iter().for_each(|steps| self.process_steps(steps))
    }

    fn process_steps(&mut self, steps: Steps) {
        let dir = steps.direction;

        for _ in 0..steps.amount {
            self.rope[0] = self.rope[0].position_in_direction(dir);
            self.update_rope();
            self.tail_visited_positions.insert(self.rope[9]);
        }
    }

    fn update_rope(&mut self) {
        for i in 1..self.rope.len() {
            let previous = self.rope[i - 1];
            let current = self.rope[i];

            if !current.neighboured_with(&previous) {
                let mut new_current = current;
                current.directions_relative_to(&previous)
                    .into_iter()
                    .for_each(|dir| new_current = new_current.position_in_direction(dir));

                self.rope[i] = new_current
            }
        }
    }

    fn num_tail_visited_positions(&self) -> usize {
        self.tail_visited_positions.len()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn position_in_direction(&self, direction: Direction) -> Position {
        match direction {
            Left => Position::new(self.x - 1, self.y),
            Right => Position::new(self.x + 1, self.y),
            Up => Position::new(self.x, self.y + 1),
            Down => Position::new(self.x, self.y - 1),
        }
    }

    fn neighboured_with(&self, other: &Position) -> bool {
        if self == other {
            return true;
        }

        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn directions_relative_to(&self, other: &Position) -> impl IntoIterator<Item=Direction> {
        [
            match self.x.cmp(&other.x) {
                Ordering::Less => Some(Right),
                Ordering::Equal => None,
                Ordering::Greater => Some(Left)
            },
            match self.y.cmp(&other.y) {
                Ordering::Less => Some(Up),
                Ordering::Equal => None,
                Ordering::Greater => Some(Down)
            }
        ]
            .into_iter()
            .flat_map(|opt| opt)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Steps {
    direction: Direction,
    amount: usize,
}

impl From<&str> for Steps {
    fn from(s: &str) -> Self {
        let split = s.split(" ").collect::<Vec<_>>();

        Steps {
            direction: Direction::from(split[0]),
            amount: split[1].parse::<usize>().expect("should be usize"),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "D" => Down,
            "U" => Up,
            "R" => Right,
            "L" => Left,
            _ => panic!("unknown direction")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p9::{Board, BoardExtended, Steps};

    #[test]
    fn examples_work() {
        let input_one = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let mut board = Board::new();
        board.process_all_steps(input_one.lines().map(Steps::from));
        assert_eq!(board.num_tail_visited_positions(), 13);

        let input_two = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        let mut ext_board = BoardExtended::new();
        ext_board.process_all_steps(input_two.lines().map(Steps::from));
        assert_eq!(ext_board.num_tail_visited_positions(), 36);
    }
}