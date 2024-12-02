use anyhow::{Error, Result};
use std::collections::HashSet;

fn find_unique_packet_location(s: &str, packet_len: usize) -> Result<usize> {
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

pub fn part1(input: &str) -> String {
    let loc = find_unique_packet_location(input, 4).unwrap();
    format!("{loc}")
}

pub fn part2(input: &str) -> String {
    let loc = find_unique_packet_location(input, 14).unwrap();
    format!("{loc}")
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
    fn test_part1() {
        assert_eq!(part1(INPUT_1), "7");
        assert_eq!(part1(INPUT_2), "5");
        assert_eq!(part1(INPUT_3), "6");
        assert_eq!(part1(INPUT_4), "10");
        assert_eq!(part1(INPUT_5), "11");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_1), "19");
        assert_eq!(part2(INPUT_2), "23");
        assert_eq!(part2(INPUT_3), "23");
        assert_eq!(part2(INPUT_4), "29");
        assert_eq!(part2(INPUT_5), "26");
    }
}
