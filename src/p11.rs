use std::collections::HashMap;
use std::ops::Mul;
use crate::input_reader::read_input;
use crate::p11::Operation::{Add, Square, Times};

pub fn solve_p11() {
    let input = read_input(11);

    let mut runner = Runner::new(input.split("\r\n\r\n").map(|s| Monkey::from(s)));
    runner.run();
    let monkey_business = runner.calculate_monkey_business();
    println!("Solution 1: {monkey_business}");

    let mut runner = StressRunner::new(input.split("\r\n\r\n").map(|s| StressMonkey::from(s)));
    runner.run();
    let monkey_business = runner.calculate_monkey_business();
    println!("Solution 2: {monkey_business}");
}

#[derive(Debug)]
struct Runner {
    monkeys: Vec<Monkey>,
    inspections: Vec<usize>,
}

impl Runner {
    fn new(monkeys: impl IntoIterator<Item=Monkey>) -> Self {
        let monkeys = monkeys.into_iter().collect::<Vec<_>>();
        Runner {
            inspections: vec![0; monkeys.len()],
            monkeys,
        }
    }

    fn run(&mut self) {
        for _ in 0..20 {
            self.run_round()
        }
    }

    fn run_round(&mut self) {
        for i in 0..self.monkeys.len() {
            let moves = self.monkeys[i]
                .process_items()
                .into_iter()
                .collect::<Vec<_>>();

            self.inspections[i] += moves.len();

            moves.into_iter().for_each(|mv| self.monkeys[mv.target].add_item(mv.item))
        }
    }

    fn calculate_monkey_business(&self) -> u128 {
        let mut sorted = self.inspections.clone();
        sorted.sort();
        sorted[sorted.len() - 1] as u128 * sorted[sorted.len() - 2] as u128
    }
}

struct StressRunner {
    monkeys: Vec<StressMonkey>,
    inspections: Vec<usize>,
}

impl StressRunner {
    fn new(monkeys: impl IntoIterator<Item=StressMonkey>) -> Self {
        let monkeys = monkeys.into_iter().collect::<Vec<_>>();
        StressRunner {
            inspections: vec![0; monkeys.len()],
            monkeys,
        }
    }

    fn run(&mut self) {
        for _ in 0..10000 {
            self.run_round()
        }
    }

    fn run_round(&mut self) {
        for i in 0..self.monkeys.len() {
            let moves = self.monkeys[i]
                .process_items()
                .into_iter()
                .collect::<Vec<_>>();

            self.inspections[i] += moves.len();

            moves.into_iter().for_each(|mv| self.monkeys[mv.target].add_item(mv.item))
        }
    }

    fn calculate_monkey_business(&self) -> u128 {
        let mut sorted = self.inspections.clone();
        sorted.sort();
        sorted[sorted.len() - 1] as u128 * sorted[sorted.len() - 2] as u128
    }
}

/// Reject humanity. Return to
#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_value: usize,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn process_items<'a>(&'a mut self) -> impl IntoIterator<Item=Move> + 'a {
        let items = self.items.drain(..).collect::<Vec<_>>();
        items.into_iter().map(|item| self.process_item(item))
    }

    fn process_item(&self, mut item: usize) -> Move {
        self.inspect_current(&mut item);
        self.get_bored(&mut item);

        match self.check(item) {
            true => Move::new(item, self.true_target),
            false => Move::new(item, self.false_target),
        }
    }

    fn inspect_current(&self, item: &mut usize) {
        *item = self.operation.calculate_new(*item)
    }

    fn get_bored(&self, item: &mut usize) {
        *item = *item / 3;
    }

    fn check(&self, item: usize) -> bool {
        item % self.test_value == 0
    }

    fn add_item(&mut self, item: usize) {
        self.items.push(item)
    }
}

struct Move {
    item: usize,
    target: usize,
}

impl Move {
    pub fn new(item: usize, target: usize) -> Self {
        Self { item, target }
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().skip(1).map(|s| s.trim());
        let items = to_item_list(lines.next().unwrap());
        let operation = Operation::from(lines.next().unwrap());
        let test_value = get_number_at_end(lines.next().unwrap());
        let true_target = get_number_at_end(lines.next().unwrap());
        let false_target = get_number_at_end(lines.next().unwrap());

        Monkey {
            items,
            operation,
            test_value,
            true_target,
            false_target,
        }
    }
}

fn to_item_list(line: &str) -> Vec<usize> {
    line
        .replace("Starting items: ", "")
        .replace(" ", "")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn get_number_at_end(line: &str) -> usize {
    line.split(" ").last().unwrap().parse::<usize>().unwrap()
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Times(usize),
    Square,
}

impl Operation {
    fn calculate_new(&self, value: usize) -> usize {
        match self {
            Add(val) => value + val,
            Times(val) => value * val,
            Square => value * value,
        }
    }

    fn calculate_new_stressful(&self, value: Remainders) -> Remainders {
        match self {
            Add(val) => value + Remainders::new(*val, &DIVIDERS),
            Times(val) => value * Remainders::new(*val, &DIVIDERS),
            Square => value.clone() * value,
        }
    }
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        let op = input.split("=").last().unwrap().replace(" ", "");

        match op.as_str() {
            "old*old" => Square,
            s if s.contains("*") => {
                let num = s.split("*").last().unwrap().parse::<usize>().unwrap();
                Times(num)
            }
            s if s.contains("+") => {
                let num = s.split("+").last().unwrap().parse::<usize>().unwrap();
                Add(num)
            }
            _ => panic!("unknown operation")
        }
    }
}

struct StressMonkey {
    items: Vec<Remainders>,
    operation: Operation,
    test_value: usize,
    true_target: usize,
    false_target: usize,
}

impl StressMonkey {
    fn process_items<'a>(&'a mut self) -> impl IntoIterator<Item=MoveStressful> + 'a {
        let items = self.items.drain(..).collect::<Vec<_>>();
        items.into_iter().map(|item| self.process_item(item))
    }

    fn process_item(&self, mut item: Remainders) -> MoveStressful {
        self.inspect_current(&mut item);

        match self.check(&item) {
            true => MoveStressful::new(item, self.true_target),
            false => MoveStressful::new(item, self.false_target),
        }
    }

    fn inspect_current(&self, item: &mut Remainders) {
        *item = self.operation.calculate_new_stressful(item.clone())
    }

    fn check(&self, item: &Remainders) -> bool {
        item.dividable_by(self.test_value)
    }

    fn add_item(&mut self, item: Remainders) {
        self.items.push(item)
    }
}

impl From<&str> for StressMonkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().skip(1).map(|s| s.trim());
        let items = to_item_list_stress(lines.next().unwrap());
        let operation = Operation::from(lines.next().unwrap());
        let test_value = get_number_at_end(lines.next().unwrap());
        let true_target = get_number_at_end(lines.next().unwrap());
        let false_target = get_number_at_end(lines.next().unwrap());

        StressMonkey {
            items,
            operation,
            test_value,
            true_target,
            false_target,
        }
    }
}

fn to_item_list_stress(line: &str) -> Vec<Remainders> {
    line
        .replace("Starting items: ", "")
        .replace(" ", "")
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .map(|val| Remainders::new(val, &DIVIDERS))
        .collect()
}

struct MoveStressful {
    item: Remainders,
    target: usize,
}

impl MoveStressful {
    pub fn new(item: Remainders, target: usize) -> Self {
        Self { item, target }
    }
}

#[derive(Clone, Debug)]
struct Remainders {
    remainders: HashMap<usize, usize>,
}

// const DIVIDERS: [usize; 4] = [23, 19, 13, 17];
const DIVIDERS: [usize; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

impl Remainders {
    fn new(input: usize, dividers: &[usize]) -> Self {
        Remainders {
            remainders: dividers
                .into_iter()
                .map(|div| (*div, input % div))
                .collect()
        }
    }

    fn dividable_by(&self, val: usize) -> bool {
        match self.remainders.get(&val) {
            None => false,
            Some(rem) => *rem == 0
        }
    }
}

// (a + b) mod n = (a mod n + b mod n) mod n
impl std::ops::Add for Remainders {
    type Output = Remainders;

    fn add(self, other: Self) -> Self::Output {
        Remainders {
            remainders: self.remainders
                .iter()
                .map(|(div, remain)| (*div, (remain + other.remainders.get(&div).unwrap()) % *div))
                .collect::<HashMap<_, _>>()
        }
    }
}

// (a * b) mod n = (a mod n) * (b mod n) mod n
impl Mul for Remainders {
    type Output = Remainders;

    fn mul(self, other: Self) -> Self::Output {
        Remainders {
            remainders: self.remainders
                .iter()
                .map(|(div, remain)| (*div, (remain * other.remainders.get(&div).unwrap()) % *div))
                .collect::<HashMap<_, _>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p11::{Monkey, DIVIDERS, Remainders, Runner, StressMonkey, StressRunner};

    #[test]
    fn create_remainders_works() {
        let remainders = Remainders::new(79, &DIVIDERS);
        println!("{:?}", remainders)
    }

    #[test]
    fn add_remainders_works() {
        let one = Remainders::new(54, &DIVIDERS);
        let two = Remainders::new(6, &DIVIDERS);
        let add = one + two;
        println!("{:?}", add)
    }

    #[test]
    fn mul_remainders_works() {
        let one = Remainders::new(54, &DIVIDERS);
        let two = Remainders::new(6, &DIVIDERS);
        let mul = one * two;
        println!("{:?}", mul)
    }

    #[test]
    fn examples_work() {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

        let mut runner = Runner::new(input.split("\n\n").map(|s| Monkey::from(s)));
        runner.run();
        assert_eq!(runner.calculate_monkey_business(), 10605);

        let mut runner = StressRunner::new(input.split("\n\n").map(|s| StressMonkey::from(s)));
        runner.run();
        assert_eq!(runner.calculate_monkey_business(), 2713310158);
    }
}