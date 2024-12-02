fn safe1(nums: &[i32]) -> u32 {
    let ascending = nums[0] < nums[1];

    let mut prev = nums[0];
    for &n in &nums[1..] {
        let diff = (prev - n).abs();

        let no_diff = diff < 1;
        let too_big_gap = diff > 3;
        let wrong_direction = (ascending && prev > n) || (!ascending && prev < n);

        if no_diff || too_big_gap || wrong_direction {
            return 0;
        }
        prev = n;
    }
    1
}

fn safe2(nums: &[i32]) -> u32 {
    if safe1(nums) == 1 {
        return 1;
    }

    for skip_idx in 0..nums.len() {
        let subset: Vec<i32> = nums
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_idx)
            .map(|(_, &n)| n)
            .collect();

        if safe1(&subset) == 1 {
            return 1;
        }
    }
    0
}

pub fn part1(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            safe1(&nums)
        })
        .sum();
    sum.to_string()
}

pub fn part2(input: &str) -> String {
    let sum: u32 = input
        .lines()
        .into_iter()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            safe2(&nums)
        })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "2");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "4");
    }
}
