use crate::input_reader::read_input;

pub fn solve_p4() {
    let input = read_input(4);

    let num_full_contains = input.lines()
        .map(RangePair::from)
        .filter(RangePair::one_contains_the_other)
        .count();

    println!("Solution 1: {num_full_contains}");

    let num_overlaps = input.lines()
        .map(RangePair::from)
        .filter(RangePair::one_overlaps_with_the_other)
        .count();

    println!("Solution 2: {num_overlaps}")
}

struct RangePair {
    first: Range,
    second: Range,
}

impl RangePair {
    fn one_contains_the_other(&self) -> bool {
        self.first.fully_contains_other(&self.second) || self.second.fully_contains_other(&self.first)
    }

    fn one_overlaps_with_the_other(&self) -> bool {
        self.first.overlaps_with_other(&self.second) || self.second.overlaps_with_other(&self.first)
    }
}

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn fully_contains_other(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_with_other(&self, other: &Range) -> bool {
        let range = self.start..=self.end;
        range.contains(&other.start) || range.contains(&other.end)
    }
}

impl From<&str> for RangePair {
    fn from(s: &str) -> Self {
        let mut ranges = s.split(",").map(Range::from);

        RangePair {
            first: ranges.next().unwrap(),
            second: ranges.next().unwrap(),
        }
    }
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let mut start_end = s.split("-").map(|part| part.parse::<usize>().unwrap());

        Range {
            start: start_end.next().unwrap(),
            end: start_end.next().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::p4::{Range, RangePair};

    #[test]
    fn range_from_str_works() {
        let s = "42-69";
        let range = Range::from(s);

        assert_eq!(42, range.start);
        assert_eq!(69, range.end);
    }

    #[test]
    fn range_pair_from_str_works() {
        let s = "42-69,420-1337";
        let range_pair = RangePair::from(s);

        assert_eq!(42, range_pair.first.start);
        assert_eq!(69, range_pair.first.end);
        assert_eq!(420, range_pair.second.start);
        assert_eq!(1337, range_pair.second.end);
    }

    #[test]
    fn fully_contains_works() {
        let first = Range {
            start: 10,
            end: 40,
        };

        let second = Range {
            start: 20,
            end: 30,
        };

        assert_eq!(first.fully_contains_other(&second), true);
        assert_eq!(first.fully_contains_other(&first), true);
        assert_eq!(second.fully_contains_other(&first), false);
    }

    #[test]
    fn overlaps_works() {
        let first = Range {
            start: 10,
            end: 20,
        };

        let second = Range {
            start: 20,
            end: 30,
        };

        let third = Range {
            start: 30,
            end: 40,
        };

        assert_eq!(first.overlaps_with_other(&second), true);
        assert_eq!(second.overlaps_with_other(&third), true);
        assert_eq!(first.overlaps_with_other(&third), false);
        assert_eq!(first.overlaps_with_other(&first), true);
    }
}