use crate::p1::solve_p1;
use crate::p2::solve_p2;
use crate::p3::solve_p3;
use crate::p4::solve_p4;

mod input_reader;
mod p1;
mod p2;
mod p3;
mod p4;

fn main() {
    let day = 4;

    println!("Day {day}");

    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        3 => solve_p3(),
        4 => solve_p4(),
        _ => println!("error")
    }
}