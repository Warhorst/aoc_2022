use crate::p1::solve_p1;
use crate::p2::solve_p2;
use crate::p3::solve_p3;
use crate::p4::solve_p4;
use crate::p5::solve_p5;
use crate::p6::solve_p6;
use crate::p7::solve_p7;

mod input_reader;
mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;

fn main() {
    let day = 7;

    println!("Day {day}");

    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        3 => solve_p3(),
        4 => solve_p4(),
        5 => solve_p5(),
        6 => solve_p6(),
        7 => solve_p7(),
        _ => println!("error")
    }
}