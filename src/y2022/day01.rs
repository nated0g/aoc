use crate::Year::Y2022;
use crate::{Problem, Year};

pub struct DayOne {}

impl DayOne {
    pub fn get_elves_totals(&self, input: &str) -> Vec<i32> {
        let elves: Vec<&str> = input.split("\n\n").collect();
        elves
            .iter()
            .map(|elf| {
                let foods: Vec<&str> = elf.split('\n').collect();
                let calories: i32 = foods
                    .iter()
                    .filter(|item| !item.is_empty())
                    .map(|food| food.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
                    .iter()
                    .sum();
                calories
            })
            .collect()
    }
}

impl Problem for DayOne {
    fn get_day(&self) -> i32 {
        1
    }
    fn get_year(&self) -> Year {
        Y2022
    }
    fn solve_part_one(&self, input: &str) -> String {
        let totals = self.get_elves_totals(input);
        let max = totals.iter().max().unwrap();
        format!("{max}")
    }

    fn solve_part_two(&self, input: &str) -> String {
        let mut totals = self.get_elves_totals(input);
        totals.sort();
        totals.reverse();
        let top_3: i32 = totals[0..3].iter().sum();

        format!("{top_3}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;
    #[test]
    fn part_one() {
        assert_eq!(DayOne {}.solve_part_one(INPUT), "24000");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayOne {}.solve_part_two(INPUT), "45000");
    }
}
