use crate::Year::Y2023;
use crate::{Problem, Year};
use std::collections::{HashMap, HashSet};

pub struct DayThree {}

impl DayThree {}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn try_from(p: Point, offset: (i32, i32), schematic: &Schematic) -> Option<Self> {
        // Only create point if it is in bounds
        if (p.0 as i32 + offset.0) < 0
            || (p.0 as i32 + offset.0) >= schematic.w as i32
            || (p.1 as i32 + offset.1) < 0
            || (p.1 as i32 + offset.1) >= schematic.h as i32
        {
            None
        } else {
            Some(Self(
                (p.0 as i32 + offset.0) as usize,
                (p.1 as i32 + offset.1) as usize,
            ))
        }
    }
}

pub struct Schematic {
    w: usize,
    h: usize,
    s: Vec<char>,
}

impl Schematic {
    fn new(input: &str) -> Self {
        let mut s = input.lines();

        let w = s.next().unwrap().len() + 1;
        let s = input.lines();
        // Pad the grid on the right with an extra '.' so we don't have to worry about
        // numbers wrapping to next line
        let s: Vec<char> = s.flat_map(|l| l.chars().chain(vec!['.'])).collect();
        let h = s.len() / w;
        Self { w, h, s }
    }

    fn get_at(&self, p: Point) -> char {
        self.s[self.idx_from_pos(p)]
    }

    fn pos_from_idx(&self, idx: usize) -> Point {
        Point(idx % self.w, idx / self.w)
    }

    fn idx_from_pos(&self, p: Point) -> usize {
        p.1 * self.w + p.0
    }

    fn add_adjacent_points_to_set(&self, p: Point, set: &mut HashSet<Point>) {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .map(|offset| Point::try_from(p, offset, self))
        .iter()
        .for_each(|p| {
            if let Some(p) = p {
                set.insert(*p);
            }
        });
    }

    fn char_is_symbol(c: char) -> bool {
        !c.is_numeric() && c != '.'
    }

    fn any_point_has_symbol(&self, set: &HashSet<Point>) -> bool {
        set.iter()
            .any(|p| Schematic::char_is_symbol(self.get_at(*p)))
    }

    fn try_get_gear_idx(&self, set: &HashSet<Point>) -> Option<usize> {
        let mut gear_pos: i32 = -1;
        set.iter().for_each(|p| {
            if self.get_at(*p) == '*' {
                gear_pos = self.idx_from_pos(*p) as i32;
            }
        });
        if gear_pos != -1 {
            Some(gear_pos as usize)
        } else {
            None
        }
    }

    fn part_str_to_num(s: Vec<char>) -> u32 {
        s.iter().collect::<String>().parse::<u32>().unwrap()
    }
}

impl Problem for DayThree {
    fn get_day(&self) -> i32 {
        3
    }
    fn get_year(&self) -> Year {
        Y2023
    }
    fn solve_part_one(&self, input: &str) -> String {
        let mut adjacent_points: HashSet<Point> = HashSet::new();
        let s = Schematic::new(input);
        let mut curr_part_num: Vec<char> = vec![];
        let mut sum = 0;
        s.s.iter().enumerate().for_each(|(i, c)| {
            if c.is_numeric() {
                let p = s.pos_from_idx(i);
                s.add_adjacent_points_to_set(p, &mut adjacent_points);
                curr_part_num.push(*c);
            } else if !curr_part_num.is_empty() {
                if s.any_point_has_symbol(&adjacent_points) {
                    let num = Schematic::part_str_to_num(curr_part_num.clone());
                    sum += num;
                }
                curr_part_num.clear();
                adjacent_points.clear();
            }
        });

        format!("{sum}")
    }

    fn solve_part_two(&self, input: &str) -> String {
        let mut potential_gears: HashMap<usize, u32> = HashMap::new();
        let mut adjacent_points: HashSet<Point> = HashSet::new();
        let s = Schematic::new(input);
        let mut curr_part_num: Vec<char> = vec![];
        let mut sum = 0;
        s.s.iter().enumerate().for_each(|(i, c)| {
            if c.is_numeric() {
                let p = s.pos_from_idx(i);
                s.add_adjacent_points_to_set(p, &mut adjacent_points);
                curr_part_num.push(*c);
            } else if !curr_part_num.is_empty() {
                if let Some(gear_pos) = s.try_get_gear_idx(&adjacent_points) {
                    let paired_gear_ratio = potential_gears.get(&gear_pos);
                    if let Some(paired_gear_ratio) = paired_gear_ratio {
                        sum +=
                            Schematic::part_str_to_num(curr_part_num.clone()) * paired_gear_ratio;
                        potential_gears.remove(&i);
                    } else {
                        potential_gears
                            .insert(gear_pos, Schematic::part_str_to_num(curr_part_num.clone()));
                    }
                }
                curr_part_num.clear();
                adjacent_points.clear();
            }
        });

        format!("{sum}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    #[test]
    fn part_one() {
        assert_eq!(DayThree {}.solve_part_one(INPUT), "4361");
    }
    #[test]
    fn part_two() {
        assert_eq!(DayThree {}.solve_part_two(INPUT), "467835");
    }
}
