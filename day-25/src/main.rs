use std::fs;
use itertools::Itertools;
use crate::Part::{Part1, Part2};

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct KeyLock {
    heights: Vec<i32>,
    is_lock: bool,
}

impl KeyLock {
    fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|l| l.chars().collect_vec())
            .collect_vec();

        let heights = (0..grid[0].len())
            .map(|x| {
                grid.iter()
                    .filter(|row| row[x] == '#')
                    .count() as i32
            })
            .collect();

        Self {
            heights,
            is_lock: grid[0][0] == '#',
        }
    }

    fn fits_with(&self, other: &KeyLock) -> bool {
        self.heights
            .iter()
            .zip(&other.heights)
            .all(|(a, b)| a + b < 8)
    }
}

fn parse_input(input: &str) -> (Vec<KeyLock>, Vec<KeyLock>) {
    input
        .split("\n\n")
        .map(KeyLock::from_str)
        .partition(|kl| kl.is_lock)
}

fn count_fitting_pairs(locks: &[KeyLock], keys: &[KeyLock]) -> i32 {
    keys.iter()
        .flat_map(|key| locks.iter().map(move |lock| key.fits_with(lock)))
        .filter(|&fits| fits)
        .count() as i32
}

fn get_value(file_path: &str, part: Part) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let (locks, keys) = parse_input(&contents);

    match part {
        Part1 => count_fitting_pairs(&locks, &keys),
        Part2 => 4,
    }
}

fn main() {
    println!("Part 1 value: {}", get_value("./input.txt", Part1));
    println!("Part 2 value: {}", get_value("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::get_value;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_value("./test.txt", Part1);
        assert_eq!(value, 3);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 3242);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}