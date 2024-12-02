use std::collections::HashSet;

pub struct DayThree {}

impl DayThree {}

struct Rucksack {
    compartments: (HashSet<char>, HashSet<char>),
}

struct ElfGroup {
    elves: Vec<Rucksack>,
}

impl ElfGroup {
    fn new(input: &[&str]) -> Self {
        let elves: Vec<Rucksack> = input.iter().map(|elf| Rucksack::new(elf)).collect();
        Self { elves }
    }
    fn find_badge(&self) -> char {
        let mut e1 = self.elves[0].rucksack_contents();
        let e2 = self.elves[1].rucksack_contents();
        let e3 = self.elves[2].rucksack_contents();
        e1.retain(|i| e2.contains(i));
        e1.retain(|i| e3.contains(i));
        e1.into_iter().collect::<Vec<char>>()[0]
    }
}

impl Rucksack {
    fn new(input: &str) -> Self {
        let mut compartments: (HashSet<char>, HashSet<char>) = (HashSet::new(), HashSet::new());
        let comp_strs = input.split_at(input.len() / 2);
        comp_strs.0.chars().for_each(|c| {
            compartments.0.insert(c);
        });
        comp_strs.1.chars().for_each(|c| {
            compartments.1.insert(c);
        });
        Self { compartments }
    }
    fn find_dupe(&self) -> char {
        let dupe: Vec<&char> = self
            .compartments
            .0
            .intersection(&self.compartments.1)
            .collect();
        *dupe[0]
    }
    fn item_to_priority(item: &char) -> i32 {
        if item.is_ascii_lowercase() {
            *item as i32 - 96 // start of lowercase ascii
        } else if item.is_ascii_uppercase() {
            *item as i32 - 64 + 26 // start of uppercase ascii, add back offset for uppercase character priority
        } else {
            0
        }
    }
    fn rucksack_contents(&self) -> HashSet<char> {
        let mut contents = self.compartments.0.clone();
        contents.extend(&self.compartments.1);
        contents
    }
}

pub fn part1(input: &str) -> String {
    let total: i32 = input
        .lines()
        .map(Rucksack::new)
        .map(|r| r.find_dupe())
        .map(|i| Rucksack::item_to_priority(&i))
        .sum();
    format!("{total}")
}

pub fn part2(input: &str) -> String {
    let elves: Vec<&str> = input.lines().collect();

    let total: i32 = elves
        .chunks(3)
        .map(ElfGroup::new)
        .map(|g| g.find_badge())
        .map(|b| Rucksack::item_to_priority(&b))
        .sum();
    format!("{total}")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "157");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "70");
    }
}
