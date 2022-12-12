use crate::p10::solve_p10;
use crate::p11::solve_p11;
use crate::p12::solve_p12;
use crate::p1::solve_p1;
use crate::p2::solve_p2;
use crate::p3::solve_p3;
use crate::p4::solve_p4;
use crate::p5::solve_p5;
use crate::p6::solve_p6;
use crate::p7::solve_p7;
use crate::p8::solve_p8;
use crate::p9::solve_p9;

mod input_reader;
mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;
mod p10;
mod p11;
mod p12;

fn main() {
    let day = 12;

    println!("Day {day}");

    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        3 => solve_p3(),
        4 => solve_p4(),
        5 => solve_p5(),
        6 => solve_p6(),
        7 => solve_p7(),
        8 => solve_p8(),
        9 => solve_p9(),
        10 => solve_p10(),
        11 => solve_p11(),
        12 => solve_p12(),
        _ => println!("error")
    }
}