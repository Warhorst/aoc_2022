use std::collections::HashSet;
use crate::input_reader::read_input;

pub fn solve_p6() {
    let input = read_input(6);
    let index = get_index_after_start_marker(&input);
    println!("Solution 1: {index}");

    let index = get_index_after_message_marker(&input);
    println!("Solution 2: {index}");
}

fn get_index_after_start_marker(input: &str) -> usize {
    get_index_after_n_distinct::<4>(input)
}

fn get_index_after_message_marker(input: &str) -> usize {
    get_index_after_n_distinct::<14>(input)
}

fn get_index_after_n_distinct<const N: usize>(input: &str) -> usize {
    input.chars()
        .collect::<Vec<_>>()
        .windows(N)
        .enumerate()
        .filter(|(_, w)| HashSet::<&char>::from_iter(w.into_iter()).len() == N)
        .map(|(i, _)| i + N)
        .next()
        .expect("no marker")
}

#[cfg(test)]
mod tests {
    use crate::p6::{get_index_after_message_marker, get_index_after_start_marker};

    #[test]
    fn find_marker_works() {
        let inputs_expected = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)
        ];

        for (input, expected) in inputs_expected.into_iter() {
            assert_eq!(get_index_after_start_marker(input), expected);
        }
    }

    #[test]
    fn find_message_works() {
        let inputs_expected = [
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)
        ];

        for (input, expected) in inputs_expected.into_iter() {
            assert_eq!(get_index_after_message_marker(input), expected);
        }
    }
}