use crate::p1::solve_p1;
use crate::p2::solve_p2;
use crate::p3::solve_p3;

mod p1;
mod p2;
mod p3;
mod input_reader;

fn main() {
    let day = 3;

    println!("Day {day}");

    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        3 => solve_p3(),
        _ => println!("error")
    }
}