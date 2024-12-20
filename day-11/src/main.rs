use std::collections::HashMap;
use std::fs;
use crate::Part::{Part1, Part2};

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct StoneTransformer {
    stones: HashMap<u64, u64>,
}

impl StoneTransformer {
    fn new(input: &str) -> Self {
        let stones = input
            .split_whitespace()
            .map(|n| (n.parse::<u64>().expect("Invalid number in input"), 1))
            .collect();
        
        Self { stones }
    }

    fn process_stone(n: u64, cache: u64) -> Vec<(u64, u64)> {
        if n == 0 {
            vec![(1, cache)]
        } else {
            let digits = n.ilog10() + 1;
            if digits % 2 == 0 {
                let power = 10u64.pow(digits / 2);
                vec![
                    (n % power, cache),
                    (n / power, cache),
                ]
            } else {
                vec![(n * MULTIPLIER, cache)]
            }
        }
    }

    fn transform(&mut self) {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (&n, &cache) in &self.stones {
            for (new_n, new_cache) in Self::process_stone(n, cache) {
                *new_stones.entry(new_n).or_default() += new_cache;
            }
        }
        self.stones = new_stones;
    }

    fn get_total(&self) -> u64 {
        self.stones.values().sum()
    }
}

const MULTIPLIER: u64 = 2024;

fn get_stone_count_after_blinks(file_path: &str, blink_count: i64, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    if part == Part1 {
        let stones: Vec<u64> = file_contents
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

        transform_stones(stones, blink_count).len()
    } else {
        let mut transformer = StoneTransformer::new(&file_contents);
    
        (0..blink_count).for_each(|_| transformer.transform());

        transformer.get_total() as usize
    }
}

fn process_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let digits = stone.to_string();
        if digits.len() % 2 == 0 {
            let mid = digits.len() / 2;
            let (first, second) = digits.split_at(mid);
            vec![
                first.parse().expect("Invalid first half"),
                second.parse().expect("Invalid second half")
            ]
        } else {
            vec![stone * 2024]
        }
    }
}

fn transform_stones(initial_stones: Vec<u64>, blink_count: i64) -> Vec<u64> {
    (0..blink_count).fold(initial_stones, |stones, _| {
        stones.into_iter()
            .flat_map(process_stone)
            .collect()
    })
}

fn main() {
    println!(
        "Part 1 value: {}",
        get_stone_count_after_blinks("./input.txt", 25, Part1)
    );
    println!(
        "Part 2 value: {}",
        get_stone_count_after_blinks("./input.txt", 75, Part2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_stone_count_after_blinks;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let stone_count = get_stone_count_after_blinks("./test.txt", 25, Part1);
        assert_eq!(stone_count, 55312);
    }
    
    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let stone_count = get_stone_count_after_blinks("./input.txt", 25, Part1);
        assert_eq!(stone_count, 199986);
    }
    
    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let stone_count = get_stone_count_after_blinks("./input.txt", 75, Part2);
        assert_eq!(stone_count, 236804088748754);
    }
}
