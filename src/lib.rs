use anyhow::*;
use std::str::FromStr;

pub mod y2022 {
    pub mod day01;
    pub mod day02;
}

#[derive(Clone, Copy, Debug)]
pub enum Year {
    Y2022 = 2022,
}

impl FromStr for Year {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let year_int: i32 = s.parse()?;
        match year_int {
            x if x == Year::Y2022 as i32 => Ok(Year::Y2022),
            _ => Err(Error::msg("Year not implemented.")),
        }
    }
}

pub trait Problem {
    fn get_day(&self) -> i32;
    fn get_year(&self) -> Year;
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;

    fn read_input(&self) -> String {
        std::fs::read_to_string(format!(
            "./src/y{}/input/day{:0>2}.txt",
            self.get_year() as i32,
            self.get_day()
        ))
        .unwrap()
    }
    fn solve(&self) {
        let input = self.read_input();
        let solution_1 = self.part_one(input.as_str());
        println!(
            "Solution for {:?}, day {}, part 1: {solution_1}",
            self.get_year(),
            self.get_day()
        );
        let solution_2 = self.part_two(input.as_str());
        println!(
            "Solution for {:?}, day {}, part 2: {solution_2}",
            self.get_year(),
            self.get_day()
        );
    }
}
