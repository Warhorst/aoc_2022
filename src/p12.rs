use std::collections::{HashMap, HashSet};
use pad::Position;
use crate::input_reader::read_input;
use crate::p12::Field::{End, Start, Val};

pub fn solve_p12() {
    let input = read_input(12);

    let board = Board::from(input.as_str());

    let len_shortest_path = board.get_len_shortest_path();
    println!("Solution 1: {len_shortest_path}");

    let len_shortest_hiking_path = board.get_len_shortest_hiking_path();
    println!("Solution 2: {len_shortest_hiking_path}");
}

#[derive(Debug)]
struct Board {
    fields: HashMap<Position, Field>,
}

impl Board {
    fn get_len_shortest_path(&self) -> usize {
        let start = self.fields
            .iter()
            .filter_map(|(pos, field)| if let Start = field {
                Some(pos)
            } else {
                None
            })
            .next().unwrap();

        let goal = self.fields
            .iter()
            .filter_map(|(pos, field)| if let End = field {
                Some(pos)
            } else {
                None
            })
            .next().unwrap();

        let path = self.a_star(*start, *goal);
        path.unwrap().len() - 1
    }

    fn get_len_shortest_hiking_path(&self) -> usize {
        let ground_nodes = self.fields
            .iter()
            .filter_map(|(pos, field)| match field {
                Start | Val(1) => Some(pos),
                _ => None
            });

        let goal = self.fields
            .iter()
            .filter_map(|(pos, field)| if let End = field {
                Some(pos)
            } else {
                None
            })
            .next().unwrap();

        ground_nodes
            .flat_map(|pos| self.a_star(*pos, *goal))
            .map(|path| path.len() - 1)
            .min()
            .unwrap()
    }

    fn a_star(&self, start: Position, goal: Position) -> Option<Vec<Position>> {
        let mut open_set: HashSet<Position> = HashSet::new();
        open_set.insert(start);

        let mut came_from: HashMap<Position, Position> = HashMap::new();

        let mut g_score: HashMap<Position, usize> = HashMap::new();
        g_score.insert(start, 0);

        let mut f_score: HashMap<Position, usize> = HashMap::new();
        f_score.insert(start, h(start, goal));

        while !open_set.is_empty() {
            let current = open_set
                .iter()
                .map(|pos| (*pos, f_score.get(pos).unwrap()))
                .min_by(|(_, val_one), (_, val_two)| val_one.cmp(&val_two))
                .unwrap().0;

            if current == goal {
                return Some(Self::construct_path(came_from, current));
            }

            open_set.remove(&current);

            for neigh in self.neighbours_of(current) {
                let tentative_g_score = g_score.get(&current).unwrap() + 1;

                if tentative_g_score < *g_score.get(&neigh).unwrap_or(&usize::MAX) {
                    came_from.insert(neigh, current);
                    g_score.insert(neigh, tentative_g_score);
                    f_score.insert(neigh, tentative_g_score + h(neigh, goal));

                    if !open_set.contains(&neigh) {
                        open_set.insert(neigh);
                    }
                }
            }
        }

        None
    }

    fn construct_path(came_from: HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
        let mut total_path = vec![current];

        while came_from.contains_key(&current) {
            current = *came_from.get(&current).unwrap();
            total_path.push(current);
        }

        total_path
    }

    fn neighbours_of<'a>(&'a self, position: Position) -> impl IntoIterator<Item=Position> + 'a {
        position.neighbours()
            .into_iter()
            .map(|n| n.position)
            .filter(|pos| self.fields.contains_key(&pos))
            .filter(move |pos| {
                let current = self.fields.get(&position).unwrap();
                let neigh = self.fields.get(&pos).unwrap();

                match (current, neigh) {
                    (Val(a), Val(b)) => match a < b {
                        true => (b - a) <= 1,
                        false => true
                    },
                    (_, Start) => false,
                    (Start, _) => true,
                    (Val(a), End) => a.abs_diff(26) <= 1,
                    (End, _) => false,
                }
            })
    }
}

fn h(current: Position, end: Position) -> usize {
    (current.x.abs_diff(end.x) + current.y.abs_diff(end.y)) as usize
}

#[derive(Debug)]
enum Field {
    Val(usize),
    Start,
    End,
}

impl From<&str> for Board {
    fn from(input: &str) -> Self {
        let height = input.lines().count();

        let fields = input.lines()
            .enumerate()
            .flat_map(|(y, line)| line
                .chars()
                .enumerate()
                .map(move |(x, c)| (Position::from((x, height - 1 - y)), match c {
                    'a'..='z' => Val(c as usize - 96),
                    'S' => Start,
                    'E' => End,
                    _ => panic!("unknown")
                }))
            )
            .collect();

        Board {
            fields
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p12::Board;

    #[test]
    fn examples_work() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        let board = Board::from(input);
        println!("{:?}", board);

        assert_eq!(board.get_len_shortest_path(), 31);
        assert_eq!(board.get_len_shortest_hiking_path(), 29);
    }
}