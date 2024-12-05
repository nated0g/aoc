pub fn part1(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    updates
        .into_iter()
        .map(|update| {
            if update_is_valid(&update, &rules) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (rules, updates) = parse_input(input);

    updates
        .into_iter()
        .map(|mut update| {
            if update_is_valid(&update, &rules) {
                return 0;
            }

            loop {
                for &(left, right) in &rules {
                    let left_pos = update.iter().position(|&x| x == left);
                    let right_pos = update.iter().rposition(|&x| x == right);

                    if let (Some(l), Some(r)) = (left_pos, right_pos) {
                        if l > r {
                            update.swap(l, r);
                        }
                    }
                }
                if update_is_valid(&update, &rules) {
                    break;
                }
            }

            update[update.len() / 2]
        })
        .sum::<u32>()
        .to_string()
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    let updates = parts.next().unwrap();

    let rules: Vec<_> = rules
        .lines()
        .map(|rule| {
            let (l, r) = rule.split_once("|").unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .collect();

    let updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|update| update.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn update_is_valid(update: &[u32], rules: &[(u32, u32)]) -> bool {
    rules.iter().all(|&(left, right)| {
        let left_pos = update.iter().position(|&x| x == left);
        let right_pos = update.iter().rposition(|&x| x == right);

        match (left_pos, right_pos) {
            (Some(l), Some(r)) => l <= r,
            _ => true,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "143");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "123");
    }
}
