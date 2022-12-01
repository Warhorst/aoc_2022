use std::fs;

pub fn solve_r1() {
    let input = fs::read_to_string("./r1_input.txt").unwrap();

    let mut index_and_calories = input
        .split("\r\n\r\n")
        .enumerate()
        .map(|(i, part)| (i, part.split("\r\n")
            .map(|val| val.parse::<usize>().unwrap())
            .sum::<usize>())
        )
        .collect::<Vec<_>>();

    index_and_calories.sort_by(|(_, cal_one), (_, cal_two)| cal_two.cmp(&cal_one));

    let sum_top_three = [index_and_calories[0], index_and_calories[1], index_and_calories[2]].into_iter().map(|(_, cal)| cal).sum::<usize>();

    println!("{:?}", sum_top_three)
}