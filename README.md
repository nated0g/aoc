# 🎄 My Advent of Code Solutions in Rust 🦀

My journey through [Advent of Code](https://adventofcode.com) in Rust. This repository contains my solutions and the tools I use to manage them.

## 📝 Structure

- Solutions are organized by year and day
- Inputs are stored in `src/inputs/y20xx/dayxx.txt` (not comitted to the repository, as they are unique to each user and not meant to be shared)
- Unit tests use the example inputs provided in each puzzle

## 🛠️ Tools

I use [aoc-cli](https://github.com/scarvalhojr/aoc-cli) to manage puzzle inputs and submissions.

You can install it with
```bash
cargo install aoc-cli
```
Then, check the `aoc-cli` repo for how to set up your session variable.

Two helper scripts are provided:

### fetch.sh
Downloads puzzle input and description:
```bash
./fetch.sh 2023 1  # Downloads day 1 of year 2023
```

### solve.sh
Runs solution and submits answer:
```bash
./solve.sh 2023 1 1  # Submits part 1 of day 1, 2023
```

## 📈 Progress

### 2022
- [x] Day 1: [Calorie Counting](src/solutions/y2022/day01.rs)
- [x] Day 2: [Rock Paper Scissors](src/solutions/y2022/day02.rs)
- [x] Day 3: [Rucksack Reorganization](src/solutions/y2022/day03.rs)
- [x] Day 4: [Camp Cleanup](src/solutions/y2022/day04.rs)
- [x] Day 5: [Supply Stacks](src/solutions/y2022/day05.rs)
- [x] Day 6: [Tuning Trouble](src/solutions/y2022/day06.rs)
- [ ] Day 7: [No Space Left On Device](src/solutions/y2022/day07.rs) (part 01 only)

### 2023
- [x] Day 1: [Trebuchet?!](src/solutions/y2023/day01.rs)
- [x] Day 2: [Cube Conundrum](src/solutions/y2023/day02.rs)
- [x] Day 3: [Gear Ratios](src/solutions/y2023/day03.rs)
- [x] Day 4: [Scratchcards](src/solutions/y2023/day04.rs)

### 2024
- [x] Day 1: [Historian Hysteria](src/solutions/y2024/day01.rs)
- [x] Day 1: [Red-Nosed Reports](src/solutions/y2024/day02.rs)

## 🚀 Running Solutions

```bash
# Run a specific solution
cargo run -- 2024 1 1  # Runs year 2024, day 1, part 1

# Run tests
cargo test
```
