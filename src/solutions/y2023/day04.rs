use std::collections::HashMap;

#[derive(Debug)]
struct Card {
    num: usize,
    actual: Vec<u32>,
    winning: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Self {
        let g: Vec<&str> = input.split(':').collect();
        let num = g[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
        let nums: Vec<&str> = g[1].split('|').collect();
        let winning = nums[0]
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();
        let actual = nums[1]
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .collect();
        Self {
            num,
            actual,
            winning,
        }
    }
    fn get_num_matching(&self) -> usize {
        self.actual
            .iter()
            .filter(|x| self.winning.contains(x))
            .count()
    }

    fn get_score(&self) -> u32 {
        let winning_count = self.get_num_matching();
        let base: u32 = 2;
        if winning_count == 0 {
            0
        } else {
            base.pow(winning_count as u32 - 1)
        }
    }

    fn increment_multipliers(&self, multipliers: &mut HashMap<usize, usize>) {
        for n in 0..self.get_num_matching() {
            let k = self.num + n + 1;
            let current_multiplier = multipliers.get(&self.num).unwrap_or(&1);
            let target_multiplier = multipliers.get(&k).unwrap_or(&1);
            multipliers.insert(k, target_multiplier + current_multiplier);
        }
    }
}
pub fn part1(input: &str) -> String {
    let score: u32 = input.lines().map(Card::new).map(|c| c.get_score()).sum();
    format!("{score}")
}

pub fn part2(input: &str) -> String {
    let mut multipliers: HashMap<usize, usize> = HashMap::new();
    let score: usize = input
        .lines()
        .map(Card::new)
        .map(|c| {
            dbg!(&multipliers);
            c.increment_multipliers(&mut multipliers);
            let m = multipliers.get(&c.num);
            if let Some(m) = m {
                *m
            } else {
                1usize
            }
        })
        .sum();

    format!("{score}")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "13");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "30");
    }
}
