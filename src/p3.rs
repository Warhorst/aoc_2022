use crate::input_reader::read_input;

pub fn solve_p3() {
    let input = read_input(3);

    let priority_sum = input.lines()
        .map(|line| {
            let half_len = line.len() / 2;

            line.chars()
                .take(half_len)
                .filter(move |char| line.chars()
                    .skip(half_len)
                    .any(|other_char| *char == other_char))
                .next()
                .unwrap()
        })
        .map(char_priority)
        .sum::<usize>();

    println!("Solution 1: {priority_sum}");

    let priority_badge_sum = input.lines()
        .map(String::from)
        .batch_three()
        .map(|batch| batch[0]
            .chars()
            .filter(|char| batch[1].contains(*char) && batch[2].contains(*char))
            .next()
            .unwrap())
        .map(char_priority)
        .sum::<usize>();

    println!("Solution 2: {priority_badge_sum}")
}

fn char_priority(char: char) -> usize {
    match char {
        c @ 'A'..='Z' => c as usize - 38,
        c @ 'a'..='z' => c as usize - 96,
        _ => panic!("unknown letter")
    }
}

struct BatchThree<I> {
    iter: I,
}

impl<I> Iterator for BatchThree<I>
    where I: Iterator<Item=String> {
    type Item = [String; 3];

    fn next(&mut self) -> Option<Self::Item> {
        Some([
            self.iter.next()?,
            self.iter.next()?,
            self.iter.next()?,
        ])
    }
}

trait BatchThreeIterator: Iterator<Item=String> + Sized {
    fn batch_three(self) -> BatchThree<Self> {
        BatchThree {
            iter: self
        }
    }
}

impl<I: Iterator<Item=String>> BatchThreeIterator for I {}

#[cfg(test)]
mod tests {
    use crate::p3::char_priority;

    #[test]
    fn test() {
        let values = (b'a'..=b'z').map(char::from).map(|char| (char, char_priority(char))).collect::<Vec<_>>();
        println!("{:?}", values);
        let values = (b'A'..=b'Z').map(char::from).map(|char| (char, char_priority(char))).collect::<Vec<_>>();
        println!("{:?}", values)
    }
}