use std::collections::HashMap;
use crate::input_reader::read_input;
use crate::p8::Direction::{Down, Left, Right, Up};

pub fn solve_p8() {
    let input = read_input(8);
    let board = Board::from(input.as_str());

    let sum_visible_trees = board.sum_visible_trees();
    println!("Solution 1: {sum_visible_trees}");

    let highest_scenic_score = board.get_highest_scenic_score();
    println!("Solution 2: {highest_scenic_score}");
}

#[derive(Debug)]
struct Board {
    heights: HashMap<Position, usize>,
}

impl Board {
    fn sum_visible_trees(&self) -> usize {
        self.heights
            .keys()
            .filter(|pos| self.position_is_visible(pos))
            .count()
    }

    fn position_is_visible(&self, pos: &Position) -> bool {
        let height = self.get_height(pos);

        pos.neighbours(self)
            .into_iter()
            .any(|neighbours| neighbours.into_iter().all(|n| self.get_height(&n) < height))
    }

    fn get_highest_scenic_score(&self) -> usize {
        self.heights
            .keys()
            .map(|pos| self.get_scenic_score(pos))
            .max()
            .expect("should have one value")
    }

    fn get_scenic_score(&self, pos: &Position) -> usize {
        let height = self.get_height(pos);

        pos.neighbours(self)
            .into_iter()
            .map(|neighbours| {
                let neighbours = neighbours.collect::<Vec<_>>();
                match neighbours.iter().take_while(|n| self.get_height(&n) < height).count() {
                    c if c < neighbours.len() => c + 1,
                    c => c
                }
            })
            .product()
    }

    fn get_height(&self, pos: &Position) -> usize {
        *self.heights.get(pos).expect("should be set")
    }
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let heights = s.lines()
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .enumerate()
                .map(move |(x, c)| (Position::new(x as isize, y as isize), c.to_digit(10).unwrap() as usize))
            )
            .collect();

        Board {
            heights,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn neighbours<'a>(&self, board: &'a Board) -> [PositionsInDirection<'a>; 4] {
        [
            PositionsInDirection::new(*self, board, Left),
            PositionsInDirection::new(*self, board, Right),
            PositionsInDirection::new(*self, board, Up),
            PositionsInDirection::new(*self, board, Down),
        ]
    }
}

struct PositionsInDirection<'a> {
    current: Position,
    board: &'a Board,
    direction: Direction,
}

impl<'a> PositionsInDirection<'a> {
    pub fn new(start: Position, board: &'a Board, direction: Direction) -> Self {
        Self {
            current: start,
            board,
            direction,
        }
    }
}

impl<'a> Iterator for PositionsInDirection<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.current.x, self.current.y);

        self.current = match self.direction {
            Up => Position::new(x, y - 1),
            Down => Position::new(x, y + 1),
            Left => Position::new(x - 1, y),
            Right => Position::new(x + 1, y)
        };

        match self.board.heights.contains_key(&self.current) {
            true => Some(self.current),
            false => None
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::p8::Board;

    #[test]
    fn board_from_string_works() {
        let input = "30373
25512
65332
33549
35390";

        let board = Board::from(input);
        println!("{:?}", board)
    }

    #[test]
    fn examples_work() {
        let input = "30373
25512
65332
33549
35390";

        let board = Board::from(input);
        assert_eq!(board.sum_visible_trees(), 21);
        assert_eq!(board.get_highest_scenic_score(), 8);
    }
}