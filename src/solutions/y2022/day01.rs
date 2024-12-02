fn get_elves_totals(input: &str) -> Vec<i32> {
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

pub fn part1(input: &str) -> String {
    let totals = get_elves_totals(input);
    let max = totals.iter().max().unwrap();
    format!("{max}")
}

pub fn part2(input: &str) -> String {
    let mut totals = get_elves_totals(input);
    totals.sort();
    totals.reverse();
    let top_3: i32 = totals[0..3].iter().sum();

    format!("{top_3}")
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
    fn test_part1() {
        assert_eq!(part1(INPUT), "24000");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "45000");
    }
}
