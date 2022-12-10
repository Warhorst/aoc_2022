use std::ops::RangeInclusive;
use crate::input_reader::read_input;
use crate::p10::Instruction::{Add, Noop};

pub fn solve_p10() {
    let input = read_input(10);

    let sum_signal_strength = sum_signal_strength(&input);
    println!("Solution 1: {sum_signal_strength}");

    let mut screen = Screen::new();
    screen.set_pixels(&input);
    println!("Solution 2");
    screen.render();
}

fn sum_signal_strength(input: &String) -> isize {
    collect_cycles_registers(input)
        .into_iter()
        .filter(|(i, _)| match i {
            20 | 60 | 100 | 140 | 180 | 220 => true,
            _ => false
        })
        .map(|(i, value)| i as isize * value)
        .sum()
}

struct Screen {
    pixels: [char; 240],
}

impl Screen {
    fn new() -> Self {
        Screen {
            pixels: ['?'; 240]
        }
    }

    fn set_pixels(&mut self, input: &String) {
        let cycles_registers = collect_cycles_registers(input);

        cycles_registers
            .into_iter()
            .for_each(|(cycle, register)| {
                self.pixels[cycle - 1] = if Self::sprite_range(cycle).contains(&(register as usize)) {
                    '#'
                } else {
                    '.'
                }
            })
    }

    fn sprite_range(cycle: usize) -> RangeInclusive<usize> {
        let position = (cycle - 1) % 40;
        let start = position.checked_sub(1).unwrap_or(0);
        let end = position + 1;
        start..=end
    }

    fn render(&self) {
        for y in 0..6 {
            for x in 0..40 {
                let val = self.pixels[y * 40 + x];
                print!("{val}");
            }
            println!();
        }
    }
}

fn collect_cycles_registers(input: &String) -> Vec<(usize, isize)> {
    input
        .lines()
        .flat_map(instructions_from)
        .enumerate()
        .fold((1, vec![]), |(mut register, mut cache), (index, instruction)| {
            cache.push((index + 1, register));

            if let Add(value) = instruction {
                register += value;
            };

            (register, cache)
        }).1
}

enum Instruction {
    Noop,
    Add(isize),
}

fn instructions_from(s: &str) -> impl IntoIterator<Item=Instruction> {
    let split = s.split(" ").collect::<Vec<_>>();

    match split[0] {
        "noop" => vec![Noop],
        "addx" => vec![Noop, Add(split[1].parse::<isize>().unwrap())],
        _ => panic!("unknown")
    }
}

#[cfg(test)]
mod tests {
    use crate::p10::{Screen, sum_signal_strength};

    #[test]
    fn examples_work() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop".to_string();

        let sum_strength = sum_signal_strength(&input);
        assert_eq!(sum_strength, 13140);

        let mut screen = Screen::new();
        screen.set_pixels(&input);
        screen.render()
    }

    #[test]
    fn screen_works() {
        let screen = Screen::new();
        screen.render();
    }
}