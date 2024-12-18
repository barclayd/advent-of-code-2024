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

## Scripts

### Start a new day

Prerequisites: 

* Get your AoC Session Cookie

This can be done by copying the value by going to https://adventofcode.com/ and opening Dev Tools.
Chrome: `Application` => `Cookies` => `https://adventofcode.com/` => `session: <value>`
Copy the value and paste it into a newly created `.env`, based on `.env.local`

```sh
brew install pup
./scripts/new-day.sh
```

* This will generate a new folder with a template ready to be worked on, including test setup and a blank a `test.txt`.

`test.txt` requires manual copying and pasting from the puzzle html at present

### Get input

```shell
./scripts/puzzle-input.sh
```

This command auto generates your `input.txt` file and places it in the latest day folder.

For example, if you are working on a solution for Day 4, it will place it in `/day-04/input.txt`, ready to be used in your solution.

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

## Continuous Integration

This project uses GitHub Actions for continuous integration. The workflow:

- Runs on every push to `main`and pull request against `main`
- Tests solution for every day

The workflow configuration can be found in `.github/workflows/ci.yml`.
These run in a parallelized matrix.

## Progress (34/50 ⭐️)

| Day | Challenge                                                           | Stars |
|-----|---------------------------------------------------------------------|----|
| 1   | [Historian Hysteria](https://adventofcode.com/2024/day/1)         | ⭐️⭐️ |
| 2   | [Red-Nosed Reports](https://adventofcode.com/2024/day/2)          | ⭐️⭐️ |
| 3   | [Mull It Over](https://adventofcode.com/2024/day/3)               | ⭐️⭐️ |
| 4   | [Ceres Search](https://adventofcode.com/2024/day/4)               | ⭐️⭐️ |
| 5   | [Print Queue](https://adventofcode.com/2024/day/5)                | ⭐️⭐️ |
| 6   | [Guard Gallivant](https://adventofcode.com/2024/day/6)            | ⭐️⭐️ |
| 7   | [Bridge Repair](https://adventofcode.com/2024/day/7)              | ⭐️⭐️ |
| 8   | [Resonant Collinearity](https://adventofcode.com/2024/day/8)      | ⭐⭐ |
| 9   | [Disk Fragmenter](https://adventofcode.com/2024/day/9) | ⭐️⭐️ |
| 10   | [Hoof It](https://adventofcode.com/2024/day/10) | ⭐️⭐ |
| 11   | [Plutonian Pebbles](https://adventofcode.com/2024/day/11) | ⭐️⭐ |
| 12   | [Garden Groups](https://adventofcode.com/2024/day/12) | ⭐️⭐ |
| 13   | [Claw Contraption](https://adventofcode.com/2024/day/13) | ⭐⭐ |
| 14   | [Restroom Redoubt](https://adventofcode.com/2024/day/14) | ⭐⭐ |
| 15   | [Warehouse Woes](https://adventofcode.com/2024/day/15) | ⭐⭐ |
| 16   | [Reindeer Maze](https://adventofcode.com/2024/day/16) | ⭐⭐ |
| 17   | [Chronospatial Computer](https://adventofcode.com/2024/day/17) | ⭐⭐ |
| 18   | [RAM Run](https://adventofcode.com/2024/day/18) | ★★ |
