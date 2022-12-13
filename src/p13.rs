use std::cmp::Ordering;
use std::cmp::Ordering::*;
use crate::input_reader::read_input;
use crate::p13::Value::{Int, List};

pub fn solve_p13() {
    let input = read_input(13);

    let sum_right_ordered_indices = sum_right_ordered_indices(input.as_str());
    println!("Solution 1: {sum_right_ordered_indices}");

    let decoder_key = compute_decoder_key(input.as_str());
    println!("Solution 2: {decoder_key}");
}

fn sum_right_ordered_indices(input: &str) -> usize {
    input.replace("\r", "").split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            ValuePair::new(
                Value::from(lines.next().unwrap()),
                Value::from(lines.next().unwrap()),
            )
        })
        .map(|pair| pair.is_ordered())
        .enumerate()
        .filter_map(|(i, ordered)| match ordered {
            true => Some(i + 1),
            false => None
        })
        .sum()
}

fn compute_decoder_key(input: &str) -> usize {
    let mut index_values = ["[[2]]", "[[6]]"]
        .into_iter()
        .chain(input.replace("\r", "").replace("\n\n", "\n").lines())
        .map(Value::from)
        .collect::<Vec<_>>();

    index_values.sort();

    index_values.into_iter()
        .enumerate()
        .filter(|(_, val)| val == &List(vec![List(vec![Int(2)])]) || val == &List(vec![List(vec![Int(6)])]))
        .map(|(i, _)| i + 1)
        .product()
}

#[derive(Debug)]
struct ValuePair {
    a: Value,
    b: Value,
}

impl ValuePair {
    pub fn new(a: Value, b: Value) -> Self {
        Self { a, b }
    }

    fn is_ordered(&self) -> bool {
        self.a.cmp(&self.b) == Less
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Value {
    Int(usize),
    List(Vec<Value>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Int(a), Int(b)) => Some(a.cmp(&b)),
            (List(a), List(b)) => compare_lists(a.clone(), b.clone()),
            (Int(_), List(_)) => List(vec![self.clone()]).partial_cmp(other),
            (List(_), Int(_)) => self.partial_cmp(&List(vec![other.clone()]))
        }
    }
}

fn compare_lists(a: Vec<Value>, b: Vec<Value>) -> Option<Ordering> {
    let len = [a.len(), b.len()].into_iter().min().unwrap();

    for i in 0..len {
        let val_a = a.get(i).unwrap();
        let val_b = b.get(i).unwrap();

        let ordering = val_a.partial_cmp(val_b);

        if let Some(Less) | Some(Greater) = ordering {
            return ordering;
        }
    }

    Some(a.len().cmp(&b.len()))
}

impl From<&str> for Value {
    fn from(input: &str) -> Self {
        if input.starts_with("[") {
            let mut level = 0;

            let cleaned = input.chars()
                .into_iter()
                .map(|c| match c {
                    '[' => {
                        level += 1;
                        match level {
                            1 => ' ',
                            _ => c
                        }
                    }
                    ']' => {
                        level -= 1;
                        match level {
                            0 => ' ',
                            _ => c
                        }
                    }
                    ',' => match level {
                        1 => '|',
                        _ => c
                    },
                    _ => c
                }).collect::<String>().replace(" ", "");

            let split = cleaned.split("|");

            List(split.into_iter().filter(|s| !s.is_empty()).map(Value::from).collect())
        } else {
            Int(input.parse::<usize>().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p13::{compute_decoder_key, sum_right_ordered_indices, Value};
    use crate::p13::Value::{Int, List};

    #[test]
    fn value_from_str_works() {
        let example = "[[1],[2,3,4]]";
        let val = Value::from(example);
        assert_eq!(val, List(vec![
            List(vec![Int(1)]),
            List(vec![Int(2), Int(3), Int(4)]),
        ]))
    }

    #[test]
    fn examples_work() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

        let sum = sum_right_ordered_indices(input);
        assert_eq!(sum, 13);

        let key = compute_decoder_key(input);
        assert_eq!(key, 140);
    }
}