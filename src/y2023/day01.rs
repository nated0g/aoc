use crate::Year::Y2023;
use crate::{Problem, Year};
use std::collections::HashMap;

pub struct DayOne {}

impl DayOne {
    pub fn replace_words(input: &str) -> u32 {
        let numbers = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        let mut matches: Vec<(i32, i32, i32)> = numbers
            .iter()
            .map(|(s, n)| {
                let n_str = n.to_string();
                let word_matches = input.match_indices(s);
                let num_matches = input.match_indices(n_str.as_str());
                let matches = word_matches.chain(num_matches);

                let min: i32 = if let Some((i, _)) = matches.clone().min() {
                    i as i32
                } else {
                    -1
                };
                let max: i32 = if let Some((i, _)) = matches.max() {
                    i as i32
                } else {
                    -1
                };
                (*n, min, max)
            })
            .filter(|t| t.1 != -1 && t.2 != -1)
            .collect();

        matches.sort_by(|a, b| a.1.cmp(&b.1));
        let first = matches[0].0;
        matches.sort_by(|a, b| b.2.cmp(&a.2));
        let last = matches[0].0;
        let num_str = format!("{first}{last}");
        num_str.parse::<u32>().unwrap()
    }
}

impl Problem for DayOne {
    fn get_day(&self) -> i32 {
        1
    }
    fn get_year(&self) -> Year {
        Y2023
    }
    fn solve_part_one(&self, input: &str) -> String {
        let output: u32 = input
            .lines()
            .map(|l| {
                let mut num = String::new();
                for f in l.chars() {
                    if let Ok(n) = f.to_string().parse::<u32>() {
                        num = n.to_string();
                        break;
                    }
                }
                for l in l.chars().rev() {
                    if let Ok(n) = l.to_string().parse::<u32>() {
                        num += n.to_string().as_mut_str();
                        break;
                    }
                }
                num.parse::<u32>().unwrap()
            })
            .sum();
        format!("{output}")
    }

    fn solve_part_two(&self, input: &str) -> String {
        let answer: u32 = input.lines().map(DayOne::replace_words).sum();
        format!("{answer}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
    const INPUT_2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    #[test]
    fn part_one() {
        assert_eq!(DayOne {}.solve_part_one(INPUT), "142");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayOne {}.solve_part_two(INPUT_2), "281");
    }
}
