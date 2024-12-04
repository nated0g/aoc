use crate::helpers::*;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (1, -1),
    (0, -1),
    (-1, 0),
    (-1, -1),
    (-1, 1),
];

fn search1(x: i32, y: i32, dir: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    let mut pos = 0;
    let mut curr_x = x;
    let mut curr_y = y;

    while pos < XMAS.len() {
        if !in_bounds(curr_x, curr_y, grid) || grid[curr_y as usize][curr_x as usize] != XMAS[pos] {
            return false;
        }
        curr_x += dir.0;
        curr_y += dir.1;
        pos += 1;
    }

    true
}

pub fn part1(input: &str) -> String {
    let grid = to_2d_array(input);

    let mut xmas_count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                for &dir in DIRS.iter() {
                    if search1(x as i32, y as i32, dir, &grid) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }

    xmas_count.to_string()
}

fn search2(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    let word1 = (grid[y - 1][x - 1], grid[y + 1][x + 1]);
    let word2 = (grid[y - 1][x + 1], grid[y + 1][x - 1]);
    (word1 == ('M', 'S') || word1 == ('S', 'M')) && (word2 == ('M', 'S') || word2 == ('S', 'M'))
}

pub fn part2(input: &str) -> String {
    let grid = to_2d_array(input);

    let mut xmas_count = 0;

    for y in 1..(grid.len() - 1) {
        for x in 1..(grid[y].len() - 1) {
            if grid[y][x] == 'A' {
                if search2(x, y, &grid) {
                    xmas_count += 1;
                }
            }
        }
    }

    xmas_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "18");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "9");
    }
}
