use crate::Year::Y2023;
use crate::{Problem, Year};

pub struct DayOne {}

impl DayOne {}

impl Problem for DayOne {
    fn get_day(&self) -> i32 {
        1
    }
    fn get_year(&self) -> Year {
        Y2023
    }
    fn solve_part_one(&self, input: &str) -> String {
        todo!("{input}")
    }

    fn solve_part_two(&self, input: &str) -> String {
        todo!("{input}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
    
"#;
    #[test]
    fn part_one() {
        assert_eq!(DayOne {}.solve_part_one(INPUT), "");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayOne {}.solve_part_two(INPUT), "");
    }
}
