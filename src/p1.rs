use crate::input_reader::read_input;

pub fn solve_p1() {
    let input = read_input(1);

    let mut calories = input
        .split("\r\n\r\n")
        .map(|part| part.split("\r\n")
            .map(|val| val.parse::<usize>().unwrap())
            .sum::<usize>()
        )
        .collect::<Vec<_>>();

    calories.sort_by(|cal_one, cal_two| cal_two.cmp(&cal_one));

    println!("Solution 1: {}", calories[0]);

    let sum_top_three = calories.into_iter().take(3).sum::<usize>();

    println!("Solution 2: {sum_top_three}")
}