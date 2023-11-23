use anyhow::*;
use aoc::y2022::day01::DayOne;
use aoc::y2022::day02::DayTwo;
use aoc::{Problem, Year};
use std::env;

fn get_problem(year: Year, day: i32) -> Result<Box<dyn Problem>> {
    match year {
        Year::Y2022 => match day {
            1 => Ok(Box::new(DayOne {})),
            2 => Ok(Box::new(DayTwo {})),
            _ => Err(Error::msg("not implemented")),
        },
        Year::Y2023 => match day {
            _ => Err(Error::msg("not implemented")),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[2];
    let day: i32 = day.parse().unwrap();
    let year = args[1].parse().unwrap();

    get_problem(year, day).unwrap().solve();
}
