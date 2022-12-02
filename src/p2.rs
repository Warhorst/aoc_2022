use std::fs::read_to_string;
use Hand::*;
use Strategy::*;

pub fn solve_p2() {
    let input = read_to_string("./p2_input.txt").unwrap();

    let score_one = input.lines()
        .map(|line| {
            let mut split = line.split(" ");
            (Hand::from(split.next().unwrap()), Hand::from(split.next().unwrap()))
        })
        .map(|(opp_hand, my_hand)| my_hand.points() + match (opp_hand, my_hand) {
            (Rock, Paper) => 6,
            (Rock, Scissors) => 0,
            (Rock, Rock) => 3,
            (Paper, Rock) => 0,
            (Paper, Scissors) => 6,
            (Paper, Paper) => 3,
            (Scissors, Rock) => 6,
            (Scissors, Paper) => 0,
            (Scissors, Scissors) => 3,
        })
        .sum::<usize>();

    println!("Solution 1: {}", score_one);

    let score_two = input.lines()
        .map(|line| {
            let mut split = line.split(" ");
            (Hand::from(split.next().unwrap()), Strategy::from(split.next().unwrap()))
        })
        .map(|(opp_hand, strategy)| strategy.points() + strategy.answer_to_hand(opp_hand).points())
        .sum::<usize>()    ;

    println!("Solution 2: {}", score_two);
}

#[derive(Copy, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl Hand {
    fn points(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" =>  Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("unknown letter")
        }
    }
}

enum Strategy {
    Win,
    Lose,
    Draw
}

impl Strategy {
    fn answer_to_hand(&self, hand: Hand) -> Hand {
        match (self, hand) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, Rock) => Rock,
            (Draw, Paper) => Paper,
            (Draw, Scissors) => Scissors,
        }
    }

    fn points(&self) -> usize {
        match self {
            Win => 6,
            Lose => 0,
            Draw => 3
        }
    }
}

impl From<&str> for Strategy {
    fn from(s: &str) -> Self {
        match s {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("unknown letter")
        }
    }
}