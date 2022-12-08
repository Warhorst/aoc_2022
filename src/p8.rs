use std::collections::HashMap;
use crate::input_reader::read_input;

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
    width: usize,
    height: usize,
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

        pos.neighbours(self.width, self.height)
            .into_iter()
            .any(|neighbours| match neighbours {
                None => true,
                Some(neighbours) => neighbours
                    .into_iter()
                    .all(|n| self.get_height(&n) < height)
            })
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

        pos.neighbours(self.width, self.height)
            .into_iter()
            .map(|neighbours| match neighbours {
                None => 0,
                Some(neighbours) => {
                    match neighbours.iter().take_while(|n| self.get_height(&n) < height).count() {
                        c if c < neighbours.len() => c + 1,
                        c => c
                    }
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
        let height = s.lines().count();
        let width = s.lines().next().expect("should have one line").len();
        let heights = s.lines()
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .enumerate()
                .map(move |(x, c)| (Position::new(x as isize, y as isize), c.to_digit(10).unwrap() as usize))
            )
            .collect();

        Board {
            width,
            height,
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

    fn neighbours(&self, board_width: usize, board_height: usize) -> [Option<Vec<Position>>; 4] {
        let (width, height) = (board_width as isize, board_height as isize);

        [
            match self.x {
                0 => None,
                _ => Some(LeftPositions::new(*self).collect())
            },
            match self.x {
                x if x == width - 1 => None,
                _ => Some(RightPositions::new(*self, board_width).collect()),
            },
            match self.y {
                0 => None,
                _ => Some(UpPositions::new(*self).collect())
            },
            match self.y {
                y if y == height - 1 => None,
                _ => Some(DownPositions::new(*self, board_height).collect())
            }
        ]
    }
}

struct LeftPositions {
    current_x: isize,
    start: Position,
}

impl LeftPositions {
    pub fn new(start: Position) -> Self {
        Self {
            current_x: start.x - 1,
            start,
        }
    }
}

impl Iterator for LeftPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x < 0 {
            None
        } else {
            let res = Position::new(self.current_x, self.start.y);
            self.current_x -= 1;
            Some(res)
        }
    }
}

struct RightPositions {
    current_x: isize,
    start: Position,
    width: usize,
}

impl RightPositions {
    pub fn new(start: Position, width: usize) -> Self {
        Self {
            current_x: start.x + 1,
            start,
            width,
        }
    }
}

impl Iterator for RightPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_x == self.width as isize {
            None
        } else {
            let res = Position::new(self.current_x, self.start.y);
            self.current_x += 1;
            Some(res)
        }
    }
}

struct UpPositions {
    current_y: isize,
    start: Position,
}

impl UpPositions {
    pub fn new(start: Position) -> Self {
        Self {
            current_y: start.y - 1,
            start
        }
    }
}

impl Iterator for UpPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y < 0 {
            None
        } else {
            let res = Position::new(self.start.x, self.current_y);
            self.current_y -= 1;
            Some(res)
        }
    }
}

struct DownPositions {
    current_y: isize,
    start: Position,
    height: usize,
}

impl DownPositions {
    pub fn new(start: Position, height: usize) -> Self {
        Self {
            current_y: start.y + 1,
            start,
            height
        }
    }
}

impl Iterator for DownPositions {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_y == self.height as isize {
            None
        } else {
            let res = Position::new(self.start.x, self.current_y);
            self.current_y += 1;
            Some(res)
        }
    }
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
        assert_eq!(board.width, 5);
        assert_eq!(board.height, 5);
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