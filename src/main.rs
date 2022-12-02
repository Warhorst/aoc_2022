use crate::p1::solve_p1;
use crate::p2::solve_p2;

mod p1;
mod p2;

fn main() {
    let day = 2;

    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("error")
    }
}
