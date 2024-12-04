pub mod solutions {
    pub mod y2022 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
        pub mod day06;
        pub mod day07;
    }
    pub mod y2023 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
    }
    pub mod y2024 {
        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
    }
}

use solutions::*;

pub fn run_solution((year, day, part): (u32, u32, u32), input: &str) -> String {
    match year {
        2022 => match (day, part) {
            (1, 1) => y2022::day01::part1(input),
            (1, 2) => y2022::day01::part2(input),
            (2, 1) => y2022::day02::part1(input),
            (2, 2) => y2022::day02::part2(input),
            (3, 1) => y2022::day03::part1(input),
            (3, 2) => y2022::day03::part2(input),
            (4, 1) => y2022::day04::part1(input),
            (4, 2) => y2022::day04::part2(input),
            (5, 1) => y2022::day05::part1(input),
            (5, 2) => y2022::day05::part2(input),
            (6, 1) => y2022::day06::part1(input),
            (6, 2) => y2022::day06::part2(input),
            (7, 1) => y2022::day07::part1(input),
            (7, 2) => y2022::day07::part2(input),
            _ => "Not implemented yet".to_string(),
        },
        2023 => match (day, part) {
            (1, 1) => y2023::day01::part1(input),
            (1, 2) => y2023::day01::part2(input),
            (2, 1) => y2023::day02::part1(input),
            (2, 2) => y2023::day02::part2(input),
            (3, 1) => y2023::day03::part1(input),
            (3, 2) => y2023::day03::part2(input),
            (4, 1) => y2023::day04::part1(input),
            (4, 2) => y2023::day04::part2(input),
            _ => "Not implemented yet".to_string(),
        },
        2024 => match (day, part) {
            (1, 1) => y2024::day01::part1(input),
            (1, 2) => y2024::day01::part2(input),
            (2, 1) => y2024::day02::part1(input),
            (2, 2) => y2024::day02::part2(input),
            (3, 1) => y2024::day03::part1(input),
            (3, 2) => y2024::day03::part2(input),
            (4, 1) => y2024::day04::part1(input),
            (4, 2) => y2024::day04::part2(input),
            _ => "Not implemented yet".to_string(),
        },
        _ => "Not implemented yet".to_string(),
    }
}

pub mod helpers {
    pub fn to_2d_array(input: &str) -> Vec<Vec<char>> {
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    }

    pub fn in_bounds(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
        y >= 0 && y < grid.len() as i32 && x >= 0 && x < grid[y as usize].len() as i32
    }
}
