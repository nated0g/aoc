use crate::Year::Y2022;
use crate::{Problem, Year};

pub struct DayThree {}

impl DayThree {}

impl Problem for DayThree {
    fn get_day(&self) -> i32 {
        3
    }
    fn get_year(&self) -> Year {
        Y2022
    }
    fn solve_part_one(&self, input: &str) -> String {
        todo!()
    }

    fn solve_part_two(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
    
"#;
    // #[test]
    // fn part_one() {
    //     assert_eq!(DayThree {}.solve_part_one(INPUT), "24000");
    // }
    // #[test]
    // fn part_two() {
    //     assert_eq!(DayThree {}.solve_part_two(INPUT), "45000");
    // }
}
