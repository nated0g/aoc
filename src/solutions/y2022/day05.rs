use regex::Regex;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
struct CrateStacks {
    stacks: BTreeMap<usize, VecDeque<char>>,
}

impl CrateStacks {
    fn new(diagram: &str) -> Self {
        let mut stacks: BTreeMap<usize, VecDeque<char>> = BTreeMap::new();
        diagram.lines().for_each(|level| {
            level.chars().enumerate().for_each(|(i, c)| {
                if c.is_ascii_uppercase() {
                    // convert from index to "stack number"
                    let stack_num = CrateStacks::idx_to_stack_num(i);
                    stacks
                        .entry(stack_num)
                        .or_insert(VecDeque::new())
                        .push_front(c);
                }
            })
        });
        Self { stacks }
    }

    fn idx_to_stack_num(idx: usize) -> usize {
        let idx_f = idx as f32;
        (idx_f - ((idx_f - 1.0) * 0.75)) as usize
    }

    fn apply_cm9000_instruction(&mut self, ins: Instruction) {
        for _ in 0..ins.count {
            let source_stack = self.stacks.get_mut(&ins.source).unwrap();
            let item = source_stack.pop_back().unwrap();
            let target_stack = self.stacks.get_mut(&ins.target).unwrap();
            target_stack.push_back(item);
        }
    }

    fn apply_cm9001_instruction(&mut self, ins: Instruction) {
        let source_stack = self.stacks.get_mut(&ins.source).unwrap();
        let to_move = &mut source_stack.split_off(source_stack.len() - ins.count);
        let target_stack = self.stacks.get_mut(&ins.target).unwrap();
        target_stack.append(to_move);
    }

    fn get_top_crates_string(&self) -> String {
        self.stacks
            .values()
            .map(|stack| stack.back().unwrap())
            .collect()
    }
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    source: usize,
    target: usize,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let (_, [count, source, target]) = caps.extract();
        Self {
            count: count.parse().unwrap(),
            source: source.parse().unwrap(),
            target: target.parse().unwrap(),
        }
    }
}
pub fn part1(input: &str) -> String {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let diagram = sections[0];
    let instructions = sections[1];
    let mut cs = CrateStacks::new(diagram);
    instructions
        .lines()
        .map(Instruction::new)
        .for_each(|i| cs.apply_cm9000_instruction(i));

    cs.get_top_crates_string()
}

pub fn part2(input: &str) -> String {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let diagram = sections[0];
    let instructions = sections[1];
    let mut cs = CrateStacks::new(diagram);
    instructions
        .lines()
        .map(Instruction::new)
        .for_each(|i| cs.apply_cm9001_instruction(i));

    cs.get_top_crates_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "CMZ");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "MCD");
    }
}
