use crate::Year::Y2024;
use crate::{Problem, Year};
use std::collections::HashMap;
use std::iter::zip;

pub struct DayOne {}

impl DayOne {}

impl Problem for DayOne {
    fn get_day(&self) -> i32 {
        1
    }
    fn get_year(&self) -> Year {
        Y2024
    }
    fn solve_part_one(&self, input: &str) -> String {
        let (mut one, mut two): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                (nums[0], nums[1])
            })
            .unzip();

        one.sort();
        two.sort();

        format!(
            "{}",
            zip(one, two).map(|(a, b)| i32::abs(a - b)).sum::<i32>()
        )
    }

    fn solve_part_two(&self, input: &str) -> String {
        let (one, two): (Vec<i32>, Vec<i32>) = input
            .lines()
            .map(|line| {
                let nums: Vec<i32> = line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();
                (nums[0], nums[1])
            })
            .unzip();

        let two_counts = two.iter().fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

        format!(
            "{}",
            one.into_iter()
                .map(|n| n * two_counts.get(&n).unwrap_or(&0))
                .sum::<i32>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;
    const INPUT_2: &str = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;
    #[test]
    fn part_one() {
        assert_eq!(DayOne {}.solve_part_one(INPUT), "11");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayOne {}.solve_part_two(INPUT_2), "31");
    }
}
