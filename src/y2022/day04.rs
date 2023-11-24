use crate::Year::Y2022;
use crate::{Problem, Year};
use std::ops::RangeInclusive;

type Assignment = RangeInclusive<u32>;

struct AssignmentPair(Assignment, Assignment);

impl AssignmentPair {
    fn new(s: &str) -> Self {
        let pair: Vec<&str> = s.split(',').collect();

        Self(
            AssignmentPair::assignment_from_str(pair[0]),
            AssignmentPair::assignment_from_str(pair[1]),
        )
    }

    fn assignment_from_str(s: &str) -> Assignment {
        let bounds: Vec<u32> = s.split('-').map(|i| i.parse::<u32>().unwrap()).collect();
        Assignment::new(bounds[0], bounds[1])
    }

    fn has_superset(&self) -> bool {
        self.0.start() <= self.1.start() && self.1.end() <= self.0.end()
            || self.1.start() <= self.0.start() && self.0.end() <= self.1.end()
    }

    fn has_overlap(&self) -> bool {
        self.0.start() <= self.1.end() && self.1.start() <= self.0.end()
    }
}

pub struct DayFour {}
impl DayFour {}
impl Problem for DayFour {
    fn get_day(&self) -> i32 {
        4
    }
    fn get_year(&self) -> Year {
        Y2022
    }
    fn solve_part_one(&self, input: &str) -> String {
        let output: i32 = input
            .lines()
            .map(AssignmentPair::new)
            .map(|a| if a.has_superset() { 1 } else { 0 })
            .sum();
        format!("{output}")
    }
    fn solve_part_two(&self, input: &str) -> String {
        let output: i32 = input
            .lines()
            .map(AssignmentPair::new)
            .map(|a| if a.has_overlap() { 1 } else { 0 })
            .sum();
        format!("{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
    #[test]
    fn part_one() {
        assert_eq!(DayFour {}.solve_part_one(INPUT), "2");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayFour {}.solve_part_two(INPUT), "4");
    }
}
