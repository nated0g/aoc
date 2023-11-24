use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[2];
    let day: i32 = day.parse().unwrap();
    let year = args[1].parse().unwrap();

    aoc::get_problem(year, day).unwrap().solve();
}
