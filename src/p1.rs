use std::fs;

pub fn solve_p1() {
    let input = fs::read_to_string("./p1_input.txt").unwrap();

    let mut calories = input
        .split("\r\n\r\n")
        .map(|part| part.split("\r\n")
            .map(|val| val.parse::<usize>().unwrap())
            .sum::<usize>()
        )
        .collect::<Vec<_>>();

    calories.sort_by(|cal_one, cal_two| cal_two.cmp(&cal_one));

    let sum_top_three = calories.into_iter().take(3).sum::<usize>();

    println!("{:?}", sum_top_three)
}