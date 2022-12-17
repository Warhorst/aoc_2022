use std::collections::HashSet;
use std::convert::identity;
use pad::Direction::{Down, Left, Right, Up};
use pad::{Direction, Position};
use crate::input_reader::read_input;

pub fn solve_p17() {
    let input = read_input(17);
    let mut runner = Runner::from(input.as_str());
    runner.run(2022);

    println!("Solution 1: {}", runner.max_y());

    // let mut runner = Runner::from(input.as_str());
    // runner.run(2022);
}

struct Runner {
    stream_iter: StreamIter,
    shape_provider: ShapeProvider,
    fields: HashSet<Position>,
    current_shape: Shape,
}

impl Runner {
    fn position_occupied(&self, pos: Position) -> bool {
        self.fields.contains(&pos)
    }

    fn run(&mut self, bound: usize) {
        let mut rocks_placed = 0;

        while rocks_placed < bound {
            match self.stream_iter.next().unwrap() {
                Push::Left => if self.can_move_in_direction(Left) {
                    self.current_shape.move_shape(Left)
                }
                Push::Right => if self.can_move_in_direction(Right) {
                    self.current_shape.move_shape(Right)
                }
            }

            if self.can_move_in_direction(Down) {
                self.current_shape.move_shape(Down)
            } else {
                self.place_shape();
                rocks_placed += 1;
            }
        }
    }

    fn can_move_in_direction(&self, dir: Direction) -> bool {
        self.current_shape.positions()
            .into_iter()
            .all(|pos| {
                let pos = pos.position_in_direction(dir);
                pos.x >= 0 &&
                    pos.x <= 6 &&
                    pos.y >= 0 &&
                    !self.position_occupied(pos)
            })
    }

    fn max_y(&self) -> isize {
        self.fields.iter().map(|pos| pos.y).max().unwrap_or(0) + 1
    }

    fn place_shape(&mut self) {
        self.current_shape.positions().into_iter().for_each(|pos| { self.fields.insert(pos); });
        self.current_shape = self.next_shape();
    }

    fn next_shape(&mut self) -> Shape {
        self.shape_provider.next_at_y(self.max_y() + 3)
    }
}

impl From<&str> for Runner {
    fn from(input: &str) -> Self {
        let mut shape_iter = ShapeProvider::new();
        let current_shape = shape_iter.next_at_y(3);

        Runner {
            stream_iter: Stream::from(input).into_iter(),
            shape_provider: shape_iter,
            // Max 5 rocks, 2022 times
            fields: HashSet::with_capacity(5 * 2022),
            current_shape,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Shape {
    positions: [Option<Position>; 5],
}

impl Shape {
    fn row() -> Self {
        let mut positions = [None; 5];
        Position::new(2, 0).iter_to(Position::new(5, 0))
            .into_iter()
            .enumerate()
            .for_each(|(i, pos)| positions[i] = Some(pos));

        Shape {
            positions
        }
    }

    fn column() -> Self {
        let mut positions = [None; 5];
        Position::new(2, 0).iter_to(Position::new(2, 3))
            .into_iter()
            .enumerate()
            .for_each(|(i, pos)| positions[i] = Some(pos));

        Shape {
            positions
        }
    }

    fn square() -> Self {
        let mut positions = [None; 5];
        Position::new(2, 0).iter_to(Position::new(3, 1))
            .into_iter()
            .enumerate()
            .for_each(|(i, pos)| positions[i] = Some(pos));

        Shape {
            positions
        }
    }

    fn cross() -> Self {
        let origin = Position::new(3, 0);

        Shape {
            positions: [
                Some(origin),
                Some(origin.position_in_direction(Up)),
                Some(origin.position_in_direction(Up).position_in_direction(Left)),
                Some(origin.position_in_direction(Up).position_in_direction(Right)),
                Some(origin.position_in_direction(Up).position_in_direction(Up)),
            ]
        }
    }

    fn el() -> Shape {
        let origin = Position::new(2, 0);

        Shape {
            positions: [
                Some(origin),
                Some(origin.position_in_direction(Right)),
                Some(origin.position_in_direction(Right).position_in_direction(Right)),
                Some(origin.position_in_direction(Right).position_in_direction(Right).position_in_direction(Up)),
                Some(origin.position_in_direction(Right).position_in_direction(Right).position_in_direction(Up).position_in_direction(Up)),
            ]
        }
    }

    fn move_shape(&mut self, dir: Direction) {
        self.positions
            .iter_mut()
            .flat_map(identity)
            .for_each(|pos| *pos = pos.position_in_direction(dir))
    }

    fn positions<'a>(&'a self) -> impl IntoIterator<Item=Position> + 'a {
        self.positions.iter()
            .flat_map(identity)
            .map(|pos| *pos)
    }

    fn set_y(&mut self, y: isize) {
        self.positions
            .iter_mut()
            .flat_map(identity)
            .for_each(|pos| pos.y += y)
    }
}

struct ShapeProvider {
    shapes: [Shape; 5],
    index: usize
}

impl ShapeProvider {
    fn new() -> Self {
        ShapeProvider {
            index: 0,
            shapes: [
                Shape::row(),
                Shape::cross(),
                Shape::el(),
                Shape::column(),
                Shape::square()
            ]
        }
    }

    fn next_at_y(&mut self, y: isize) -> Shape {
        let mut next = self.shapes[self.index];

        self.index += 1;

        if self.index == self.shapes.len() {
            self.index = 0
        }

        next.set_y(y);
        next
    }
}

#[derive(Debug)]
struct Stream {
    pushes: Vec<Push>,
}

impl Stream {
    fn len(&self) -> usize {
        self.pushes.len()
    }

    fn get_push_at(&self, index: usize) -> Push {
        self.pushes[index]
    }
}

struct StreamIter {
    index: usize,
    stream: Stream,
}

impl StreamIter {
    fn new(stream: Stream) -> Self {
        StreamIter {
            index: 0,
            stream,
        }
    }
}

impl IntoIterator for Stream {
    type Item = Push;
    type IntoIter = StreamIter;

    fn into_iter(self) -> Self::IntoIter {
        StreamIter::new(self)
    }
}

impl Iterator for StreamIter {
    type Item = Push;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.stream.len() {
            self.index = 0
        }

        let res = self.stream.get_push_at(self.index);
        self.index += 1;
        Some(res)
    }
}

#[derive(Copy, Clone, Debug)]
enum Push {
    Left,
    Right,
}

impl From<&str> for Stream {
    fn from(input: &str) -> Self {
        Stream {
            pushes: input
                .chars()
                .map(|c| match c {
                    '<' => Push::Left,
                    '>' => Push::Right,
                    _ => panic!("unknown")
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p17::{Runner, Shape, Stream};

    #[test]
    fn stream_from_str_works() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let stream = Stream::from(input);
        println!("{:?}", stream)
    }

    #[test]
    fn create_shapes_works() {
        [
            Shape::row(),
            Shape::column(),
            Shape::square(),
            Shape::cross(),
            Shape::el()
        ]
            .into_iter()
            .for_each(|shape| println!("{:?}", shape))
    }

    #[test]
    fn example_works() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let mut runner = Runner::from(input);
        runner.run(2022);
        assert_eq!(runner.max_y(), 3068);

        // let foo = 1_000_000_000_000;
        //
        // let mut runner = Runner::from(input);
        // runner.run(1000000000000);
        // assert_eq!(runner.max_y(), 1514285714288);
    }
}