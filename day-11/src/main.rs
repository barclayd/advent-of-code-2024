use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn get_stone_count_after_blinks(file_path: &str, blink_count: i64, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let stones: Vec<u64> = file_contents
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    if part == Part1 {
        transform_stones(stones, blink_count).len()
    } else {
        4
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
        get_stone_count_after_blinks("./input.txt", 25, Part2)
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
    fn returns_expected_value_test_data_for_part_2() {
        let stone_count = get_stone_count_after_blinks("./test.txt", 25, Part2);
        assert_eq!(stone_count, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let stone_count = get_stone_count_after_blinks("./input.txt", 25, Part2);
        assert_eq!(stone_count, 4);
    }
}
