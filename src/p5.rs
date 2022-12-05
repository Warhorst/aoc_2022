use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::input_reader::read_input;

pub fn solve_p5() {
    let input = read_input(5);
    let mut split = input.split("\r\n\r\n");

    let stacks_input = split.next().unwrap();
    let steps_input = split.next().unwrap();

    let mut stacks = Stacks::from(stacks_input);

    steps_input.lines()
        .map(Step::from)
        .for_each(|step| stacks.execute_step_9000(step));

    let top = stacks.top();

    println!("Solution 1: {top}");

    let mut stacks = Stacks::from(stacks_input);

    steps_input.lines()
        .map(Step::from)
        .for_each(|step| stacks.execute_step_9001(step));

    let top = stacks.top();

    println!("Solution 2: {top}");
}

#[derive(Debug)]
struct Stacks {
    stacks: HashMap<usize, Vec<char>>,
}

impl Stacks {
    fn new() -> Self {
        Stacks {
            stacks: HashMap::new()
        }
    }

    fn insert(&mut self, index: usize, val: char) {
        let stack = match self.stacks.entry(index) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Vec::new())
        };

        stack.push(val)
    }

    fn execute_step_9000(&mut self, step: Step) {
        let taken = self.take_amount_from(step.amount, step.start);
        self.place_on_stack(step.target, taken)
    }

    fn execute_step_9001(&mut self, step: Step) {
        let taken = self.take_amount_from(step.amount, step.start);
        self.place_on_stack(step.target, taken.into_iter().rev())
    }

    fn take_amount_from(&mut self, amount: usize, stack: usize) -> Vec<char> {
        (0..amount)
            .map(|_| self.stacks.get_mut(&stack).expect("should be set").pop().unwrap())
            .collect()
    }

    fn place_on_stack(&mut self, target: usize, values: impl IntoIterator<Item=char>) {
        values.into_iter().for_each(|c| self.stacks.get_mut(&target).expect("should be set").push(c))
    }

    fn top(&self) -> String {
        (1..=self.stacks.len())
            .map(|i| self.stacks.get(&i).expect("should be set"))
            .map(|stack| stack.last().expect("should be set"))
            .collect()
    }
}

impl From<&str> for Stacks {
    fn from(input: &str) -> Self {
        let mut stacks = Stacks::new();

        input.lines()
            .rev()
            .skip(1)
            .for_each(|line| line.replace("[", "")
                .replace("]", "")
                .replace("    ", "?")
                .replace(" ", "")
                .chars()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '?' => None,
                    _ => Some((i + 1, c))
                })
                .for_each(|(i, c)| stacks.insert(i, c))
            );

        stacks
    }
}

struct Step {
    start: usize,
    target: usize,
    amount: usize,
}

impl From<&str> for Step {
    fn from(input: &str) -> Self {
        let mut values = input
            .split(" ")
            .filter_map(|part| match part.parse::<usize>() {
                Ok(val) => Some(val),
                _ => None
            });

        Step {
            amount: values.next().unwrap(),
            start: values.next().unwrap(),
            target: values.next().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p5::{Stacks, Step};

    #[test]
    fn step_from_str_works() {
        let string = "move 4 from 2 to 1";
        let step = Step::from(string);

        assert_eq!(2, step.start);
        assert_eq!(1, step.target);
        assert_eq!(4, step.amount);
    }

    #[test]
    fn stacks_from_str_works() {
        let string = "\
[T] [V]                     [W]
[V] [C] [P] [D]             [B]
[J] [P] [R] [N] [B]         [Z]
[W] [Q] [D] [M] [T]     [L] [T]
[N] [J] [H] [B] [P] [T] [P] [L]
[R] [D] [F] [P] [R] [P] [R] [S] [G]
[M] [W] [J] [R] [V] [B] [J] [C] [S]
[S] [B] [B] [F] [H] [C] [B] [N] [L]
 1   2   3   4   5   6   7   8   9";

        let stacks = Stacks::from(string);
        println!("{:?}", stacks)
    }

    #[test]
    fn execute_step_works() {
        let string = "\
[T] [V]                     [W]
[V] [C] [P] [D]             [B]
[J] [P] [R] [N] [B]         [Z]
[W] [Q] [D] [M] [T]     [L] [T]
[N] [J] [H] [B] [P] [T] [P] [L]
[R] [D] [F] [P] [R] [P] [R] [S] [G]
[M] [W] [J] [R] [V] [B] [J] [C] [S]
[S] [B] [B] [F] [H] [C] [B] [N] [L]
 1   2   3   4   5   6   7   8   9";

        let mut stacks = Stacks::from(string);
        stacks.execute_step_9000(Step { start: 1, target: 6, amount: 3 });
        println!("{:?}", stacks)
    }

    #[test]
    fn top_works() {
        let string = "\
[T] [V]                     [W]
[V] [C] [P] [D]             [B]
[J] [P] [R] [N] [B]         [Z]
[W] [Q] [D] [M] [T]     [L] [T]
[N] [J] [H] [B] [P] [T] [P] [L]
[R] [D] [F] [P] [R] [P] [R] [S] [G]
[M] [W] [J] [R] [V] [B] [J] [C] [S]
[S] [B] [B] [F] [H] [C] [B] [N] [L]
 1   2   3   4   5   6   7   8   9";

        let stacks = Stacks::from(string);
        println!("{}", stacks.top())
    }
}