use crate::Year::Y2023;
use crate::{Problem, Year};
use regex::Regex;

pub struct DayTwo {}

impl DayTwo {}

#[derive(Debug)]
pub struct Game {
    num: u32,
    revealed_r: Vec<u32>,
    revealed_g: Vec<u32>,
    revealed_b: Vec<u32>,
}

impl Game {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"(\d+)").unwrap();
        let num = re.find(input).unwrap().as_str().parse::<u32>().unwrap();

        let mut revealed_r: Vec<u32> = vec![];
        let mut revealed_g: Vec<u32> = vec![];
        let mut revealed_b: Vec<u32> = vec![];

        let re = Regex::new(r"(\d+) red").unwrap();
        for cap in re.captures_iter(input) {
            let num = cap[1].parse::<u32>().unwrap();
            revealed_r.push(num);
        }

        let re = Regex::new(r"(\d+) green").unwrap();
        for cap in re.captures_iter(input) {
            let num = cap[1].parse::<u32>().unwrap();
            revealed_g.push(num);
        }

        let re = Regex::new(r"(\d+) blue").unwrap();
        for cap in re.captures_iter(input) {
            let num = cap[1].parse::<u32>().unwrap();
            revealed_b.push(num);
        }

        Self {
            num,
            revealed_r,
            revealed_g,
            revealed_b,
        }
    }
    fn meets_bar(&self) -> bool {
        let red_ok = self.revealed_r.iter().all(|n| *n <= 12);
        let grn_ok = self.revealed_g.iter().all(|n| *n <= 13);
        let blu_ok = self.revealed_b.iter().all(|n| *n <= 14);

        dbg!(&self);
        red_ok && grn_ok && blu_ok
    }

    fn power_of_min_set(&self) -> u32 {
        let r = self.revealed_r.iter().max().unwrap_or(&0);
        let g = self.revealed_g.iter().max().unwrap_or(&0);
        let b = self.revealed_b.iter().max().unwrap_or(&0);
        dbg!(r, g, b);
        r * g * b
    }
}

impl Problem for DayTwo {
    fn get_day(&self) -> i32 {
        2
    }
    fn get_year(&self) -> Year {
        Y2023
    }
    fn solve_part_one(&self, input: &str) -> String {
        let output: u32 = input
            .lines()
            .collect::<Vec<_>>()
            .iter()
            .map(|l| Game::new(l))
            .map(|g| {
                if g.meets_bar() {
                    dbg!(g.num);
                    g.num
                } else {
                    0
                }
            })
            .sum();

        format!("{output}")
    }

    fn solve_part_two(&self, input: &str) -> String {
        let output: u32 = input
            .lines()
            .collect::<Vec<_>>()
            .iter()
            .map(|l| Game::new(l))
            .map(|g| g.power_of_min_set())
            .sum();

        format!("{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    #[test]
    fn part_one() {
        assert_eq!(DayTwo {}.solve_part_one(INPUT), "8");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayTwo {}.solve_part_two(INPUT), "2286");
    }
}
