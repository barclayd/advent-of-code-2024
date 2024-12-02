use std::fs;

#[derive(PartialEq)]
enum LevelStatus {
    Increasing,
    Decreasing,
    Invalid
}

fn parse_lines_to_vec(file_path: &str) -> Vec<Vec<i32>> {
    let file_contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Should be a valid number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn get_status_for_level(sequence: &Vec<i32>) -> LevelStatus {
    if sequence.len() < 2 {
        return LevelStatus::Invalid;
    }

    let first_diff = sequence[1] - sequence[0];
    let level_status = if first_diff > 0 {
        LevelStatus::Increasing
    } else {
        LevelStatus::Decreasing
    };

    for window in sequence.windows(2) {
        let diff = window[1] - window[0];
        if diff < -3 || diff > 3 || diff == 0 {
            return LevelStatus::Invalid;
        }
        if diff < 0 && level_status == LevelStatus::Increasing  {
            return LevelStatus::Invalid
        }
        if diff > 0 && level_status == LevelStatus::Decreasing  {
            return LevelStatus::Invalid
        }
    }

    level_status
}

fn get_number_of_safe_levels(file_path: &str) -> i32 {
    let lines = parse_lines_to_vec(file_path);
    lines.iter()
        .filter(|line| get_status_for_level(line) != LevelStatus::Invalid)
        .count() as i32
}

fn main() {
    println!("Total number of safe levels: {}", get_number_of_safe_levels("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::get_number_of_safe_levels;

    #[test]
    fn returns_expected_number_of_safe_levels_for_test_data() {
        let number_of_safe_levels = get_number_of_safe_levels("./test.txt");
        assert_eq!(number_of_safe_levels, 2);
    }

    #[test]
    fn returns_expected_number_of_safe_levels_for_input_data() {
        let number_of_safe_levels = get_number_of_safe_levels("./input.txt");
        assert_eq!(number_of_safe_levels, 306);
    }
}