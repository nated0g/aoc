use crate::Year::Y2022;
use crate::{Problem, Year};
use anyhow::{Error, Result};
use std::collections::HashSet;

pub struct DaySix {}

impl DaySix {
    pub fn find_unique_packet_location(&self, s: &str, packet_len: usize) -> Result<usize> {
        let mut packet: HashSet<char> = HashSet::with_capacity(packet_len);
        let mut back: usize = 0;

        for front in 0..=s.len() {
            if front - back >= packet_len {
                back += 1;
                packet.clear();
                s[back..=front].chars().for_each(|c| {
                    packet.insert(c);
                });
                if packet.len() >= packet_len {
                    return Ok(front + 1);
                }
            }
        }
        Err(Error::msg(
            "Unique packet of specified length does not exist in input.",
        ))
    }
}

impl Problem for DaySix {
    fn get_day(&self) -> i32 {
        6
    }
    fn get_year(&self) -> Year {
        Y2022
    }
    fn solve_part_one(&self, input: &str) -> String {
        let loc = self.find_unique_packet_location(input, 4).unwrap();
        format!("{loc}")
    }

    fn solve_part_two(&self, input: &str) -> String {
        let loc = self.find_unique_packet_location(input, 14).unwrap();
        format!("{loc}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_1: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;
    const INPUT_2: &str = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;
    const INPUT_3: &str = r#"nppdvjthqldpwncqszvftbrmjlhg"#;
    const INPUT_4: &str = r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#;
    const INPUT_5: &str = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#;
    #[test]
    fn part_one() {
        assert_eq!(DaySix {}.solve_part_one(INPUT_1), "7");
        assert_eq!(DaySix {}.solve_part_one(INPUT_2), "5");
        assert_eq!(DaySix {}.solve_part_one(INPUT_3), "6");
        assert_eq!(DaySix {}.solve_part_one(INPUT_4), "10");
        assert_eq!(DaySix {}.solve_part_one(INPUT_5), "11");
    }
    #[test]
    fn part_two() {
        assert_eq!(DaySix {}.solve_part_two(INPUT_1), "19");
        assert_eq!(DaySix {}.solve_part_two(INPUT_2), "23");
        assert_eq!(DaySix {}.solve_part_two(INPUT_3), "23");
        assert_eq!(DaySix {}.solve_part_two(INPUT_4), "29");
        assert_eq!(DaySix {}.solve_part_two(INPUT_5), "26");
    }
}
