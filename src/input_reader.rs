use std::fs::read_to_string;

pub fn read_input(day: usize) -> String {
    read_to_string(format!("./p{day}_input.txt")).expect("failed to read input")
}