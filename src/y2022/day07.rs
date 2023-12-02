use crate::Year::Y2022;
use crate::{Problem, Year};
use anyhow::{Error, Result};

pub struct DaySeven {}

#[derive(Debug, PartialEq)]
enum Command {
    Ls(u32),
    CdUp,
    CdDown,
}

impl Command {
    fn from_str(s: &str) -> Result<Self> {
        let command = &s[0..2];
        let rest = &s[2..];
        match command {
            "ls" => Ok(Self::Ls(Command::sum_file_sizes(rest))),
            "cd" => {
                if rest == " .." {
                    Ok(Self::CdUp)
                } else {
                    Ok(Self::CdDown)
                }
            }
            _ => Err(Error::msg("invalid command")),
        }
    }
    fn sum_file_sizes(input: &str) -> u32 {
        input
            .lines()
            .filter(|s| !s.starts_with("dir") && !s.is_empty())
            .map(|s| {
                s.split_whitespace().collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .unwrap()
            })
            .sum()
    }
}

impl DaySeven {}

impl Problem for DaySeven {
    fn get_day(&self) -> i32 {
        7
    }
    fn get_year(&self) -> Year {
        Y2022
    }
    fn solve_part_one(&self, input: &str) -> String {
        let commands: Vec<Command> = input
            .split('$')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .map(|s| Command::from_str(s).unwrap())
            .collect();

        const LIMIT: u32 = 100000;
        let mut stack: Vec<u32> = vec![];
        let mut total_under_limit = 0;
        let mut curr = 0;

        commands.iter().for_each(|c| match c {
            Command::CdDown => {}
            Command::Ls(size) => {
                stack.push(curr);
                curr = *size;
            }
            Command::CdUp => {
                if curr < LIMIT {
                    total_under_limit += curr;
                }
                curr += stack.pop().unwrap();
            }
        });
        stack.push(curr);
        stack.iter().rev().for_each(|dir| {
            if *dir < LIMIT {
                total_under_limit += dir;
            }
            curr += dir;
        });
        dbg!(curr);
        format!("{total_under_limit}")
    }
    fn solve_part_two(&self, input: &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;
    #[test]
    fn part_one() {
        assert_eq!(DaySeven {}.solve_part_one(INPUT), "95437");
    }
    #[test]
    fn part_two() {
        assert_eq!(DaySeven {}.solve_part_two(INPUT), "");
    }
}
