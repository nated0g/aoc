use crate::Year::Y2022;
use crate::{Problem, Year};
use std::str::FromStr;

pub struct DayTwo {}

impl DayTwo {}

struct Match {
    us: Choice,
    them: Choice,
}

impl Match {
    pub fn won(&self) -> bool {
        self.us.wins_against() == self.them
    }
    pub fn tied(&self) -> bool {
        self.us == self.them
    }
    pub fn get_score(self) -> i32 {
        let match_score = if self.won() {
            6
        } else if self.tied() {
            3
        } else {
            0
        };
        match_score + self.us as i32
    }
    pub fn from_instructions(s: &str) -> Self {
        let instructions: Vec<&str> = s.split_whitespace().collect();
        let them = Choice::from_str(instructions[0]).unwrap();
        let outcome = Outcome::from_str(instructions[1]).unwrap();
        let us = match outcome {
            Outcome::Win => them.loses_to(),
            Outcome::Loss => them.wins_against(),
            Outcome::Draw => them.clone(),
        };
        Self { us, them }
    }
}

impl FromStr for Match {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choices: Vec<&str> = s.split_whitespace().collect();
        Ok(Match {
            them: Choice::from_str(choices[0]).unwrap(),
            us: Choice::from_str(choices[1]).unwrap(),
        })
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    pub fn wins_against(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }
    pub fn loses_to(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }
}

impl FromStr for Choice {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

impl Problem for DayTwo {
    fn get_day(&self) -> i32 {
        2
    }
    fn get_year(&self) -> Year {
        Y2022
    }
    fn part_one(&self, input: &str) -> String {
        let total: i32 = input
            .lines()
            .map(|line| Match::from_str(line).unwrap().get_score())
            .sum();
        format!("{total}")
    }

    fn part_two(&self, input: &str) -> String {
        let total: i32 = input
            .lines()
            .map(|line| Match::from_instructions(line).get_score())
            .sum();
        format!("{total}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"A Y
B X
C Z"#;
    #[test]
    fn part_one() {
        assert_eq!(DayTwo {}.part_one(INPUT), "15");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayTwo {}.part_two(INPUT), "12");
    }
}
