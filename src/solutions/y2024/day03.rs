use regex::Regex;

pub fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let x: i32 = cap[1].parse().unwrap();
            let y: i32 = cap[2].parse().unwrap();
            x * y
        })
        .sum::<i32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    let mut enabled = true;

    re.captures_iter(input)
        .filter_map(|cap| {
            if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
                if enabled {
                    let x = x.as_str().parse::<i32>().unwrap();
                    let y = y.as_str().parse::<i32>().unwrap();
                    Some(x * y)
                } else {
                    None
                }
            } else {
                match &cap[0] {
                    "don't()" => enabled = false,
                    "do()" => enabled = true,
                    _ => {}
                }
                None
            }
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "161");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), "48");
    }
}
