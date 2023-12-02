use anyhow::*;
use std::str::FromStr;

pub mod y2022;
pub mod y2023;

#[derive(Clone, Copy, Debug)]
pub enum Year {
    Y2022 = 2022,
    Y2023 = 2023,
}

impl FromStr for Year {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let year_int: i32 = s.parse()?;
        match year_int {
            x if x == Year::Y2022 as i32 => Ok(Year::Y2022),
            x if x == Year::Y2023 as i32 => Ok(Year::Y2023),
            _ => Err(Error::msg("Year not implemented.")),
        }
    }
}

pub trait Problem {
    fn get_day(&self) -> i32;
    fn get_year(&self) -> Year;
    fn solve_part_one(&self, input: &str) -> String;
    fn solve_part_two(&self, input: &str) -> String;

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
        let solution_1 = self.solve_part_one(input.as_str());
        println!(
            "Solution for {:?}, day {}, part 1: {solution_1}",
            self.get_year(),
            self.get_day()
        );
        let solution_2 = self.solve_part_two(input.as_str());
        println!(
            "Solution for {:?}, day {}, part 2: {solution_2}",
            self.get_year(),
            self.get_day()
        );
    }
}

pub fn get_problem(year: Year, day: i32) -> Result<Box<dyn Problem>> {
    match year {
        Year::Y2022 => {
            use self::y2022::*;
            match day {
                1 => Ok(Box::new(DayOne {})),
                2 => Ok(Box::new(DayTwo {})),
                3 => Ok(Box::new(DayThree {})),
                4 => Ok(Box::new(DayFour {})),
                5 => Ok(Box::new(DayFive {})),
                6 => Ok(Box::new(DaySix {})),
                7 => Ok(Box::new(DaySeven {})),
                _ => Err(Error::msg("not implemented")),
            }
        }
        Year::Y2023 => {
            use self::y2023::*;
            match day {
                1 => Ok(Box::new(DayOne {})),
                2 => Ok(Box::new(DayTwo {})),
                _ => Err(Error::msg("not implemented")),
            }
        }
    }
}
