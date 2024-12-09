# Advent of Code 2024

[![ci: passing](https://img.shields.io/badge/ci-passing-brightgreen?style=for-the-badge)](https://github.com/barclayd/advent-of-code-2024/actions)
&nbsp;
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=orange)](https://www.rust-lang.org/)

Solutions for [Advent of Code 2024](https://adventofcode.com/2024) written in Rust 🦀

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)
- Cargo (comes with Rust)

## Getting Started

```bash
git clone https://github.com/barclayd/advent-of-code-2024.git
cd advent-of-code-2024
cargo build
cargo test
```

## Start a new day

Prerequisites: 

* Get your AoC Session Cookie

This can be done by copying the value by going to https://adventofcode.com/ and opening Dev Tools.
Chrome: "Application" => "Cookies" => "https://adventofcode.com/" => "session: value"
Copy the value and paste it into the .env

```sh
brew install pup
./scripts/new-day.sh
```

* This will generate a new folder with a template ready to be worked on, including test setup and a blank a `test.txt`.

`test.txt` requires manual copying and pasting from the puzzle html at present

### Get input

```shell
./scripts/load-input.sh
```

This command auto generates your `input.txt` file.

It is to be run when you have understood the puzzle and your tests locally for `test.txt` are passing.

## Project Structure

```
advent-of-code-2024/
├── day-01/
├── day-02/
├── day-03/
...
└── README.md
```

## Progress (14/50 ⭐️)

| Day | Stars | Link                                                                |
|-----|-------|---------------------------------------------------------------------|
| 1   | ⭐️⭐️  | [Day 1: Historian Hysteria](https://adventofcode.com/2024/day/1)    |
| 2   | ⭐️⭐️  | [Day 2: Red-Nosed Reports](https://adventofcode.com/2024/day/2)     |
| 3   | ⭐️⭐️  | [Day 3: Mull It Over](https://adventofcode.com/2024/day/3)          |
| 4   | ⭐️⭐️  | [Day 4: Ceres Search](https://adventofcode.com/2024/day/4)          |
| 5   | ⭐️⭐️  | [Day 5: Print Queue](https://adventofcode.com/2024/day/5)           |
| 6   | ⭐️⭐️  | [Day 6: Guard Gallivant](https://adventofcode.com/2024/day/6)       |
| 7   | ⭐️⭐️  | [Day 7: Bridge Repair](https://adventofcode.com/2024/day/7)         |
