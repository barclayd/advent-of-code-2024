use std::fs;

#[derive(PartialEq, Debug)]
enum LevelStatus {
    Increasing,
    Decreasing,
    Invalid,
}

fn parse_lines_to_vec(file_path: &str) -> Vec<Vec<i32>> {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

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
        let is_invalid = diff > 3 || diff < -3 || diff == 0 
            || (diff < 0 && level_status == LevelStatus::Increasing)
            || (diff > 0 && level_status == LevelStatus::Decreasing);

        if is_invalid {
            return LevelStatus::Invalid;
        }
    }

    level_status
}

fn get_status_for_level_with_problem_dampener(sequence: &Vec<i32>) -> LevelStatus {
    if sequence.len() < 2 {
        return LevelStatus::Invalid;
    }

    if get_status_for_level(sequence) != LevelStatus::Invalid {
        return get_status_for_level(sequence);
    }

    for i in 0..sequence.len() {
        let mut test_sequence = sequence.clone();
        test_sequence.remove(i);

        let updated_sequence = get_status_for_level(&test_sequence);

        if updated_sequence != LevelStatus::Invalid {
            return updated_sequence;
        }
    }

    LevelStatus::Invalid
}

fn get_number_of_safe_levels(file_path: &str) -> i32 {
    let lines = parse_lines_to_vec(file_path);
    lines
        .iter()
        .filter(|line| get_status_for_level(line) != LevelStatus::Invalid)
        .count() as i32
}

fn get_number_of_safe_levels_with_problem_dampener(file_path: &str) -> i32 {
    let lines = parse_lines_to_vec(file_path);
    lines
        .iter()
        .filter(|line| get_status_for_level_with_problem_dampener(line) != LevelStatus::Invalid)
        .count() as i32
}

fn main() {
    println!(
        "Total number of safe levels: {}",
        get_number_of_safe_levels("./input.txt")
    );
    println!(
        "Total number of safe levels with problem dampener: {}",
        get_number_of_safe_levels_with_problem_dampener("./input.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::{get_number_of_safe_levels, get_number_of_safe_levels_with_problem_dampener};

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

    #[test]
    fn returns_expected_number_of_safe_levels_with_problem_dampener_for_test_data() {
        let number_of_safe_levels_with_problem_dampener =
            get_number_of_safe_levels_with_problem_dampener("./test.txt");
        assert_eq!(number_of_safe_levels_with_problem_dampener, 4);
    }

    #[test]
    fn returns_expected_number_of_safe_levels_with_problem_dampener_for_input_data() {
        let number_of_safe_levels_with_problem_dampener =
            get_number_of_safe_levels_with_problem_dampener("./input.txt");
        assert_eq!(number_of_safe_levels_with_problem_dampener, 366);
    }
}
