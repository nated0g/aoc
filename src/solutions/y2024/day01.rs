use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let (mut one, mut two): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    one.sort();
    two.sort();

    format!(
        "{}",
        std::iter::zip(one, two)
            .map(|(a, b)| i32::abs(a - b))
            .sum::<i32>()
    )
}

pub fn part2(input: &str) -> String {
    let (one, two): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    let two_counts = two.iter().fold(HashMap::new(), |mut acc, &x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    format!(
        "{}",
        one.into_iter()
            .map(|n| n * two_counts.get(&n).unwrap_or(&0))
            .sum::<i32>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"3   4
    4   3
    2   5
    1   3
    3   9
    3   3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "11");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "31");
    }
}
