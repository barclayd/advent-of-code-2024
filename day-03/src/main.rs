use std::fs;
use regex::Regex;

fn get_instructions_from_string(input: &str) -> Result<Vec<(i32, i32)>, String> {
    let re = Regex::new(r"mul\(\s*(\d+)\s*,\s*(\d+)\s*\)").unwrap();

    re.captures_iter(input).map(|c| {
        let first_digit = c.get(1).unwrap().as_str();
        let second_digit = c.get(2).unwrap().as_str();

        match (first_digit.parse::<i32>(), second_digit.parse::<i32>()) {
            (Ok(x), Ok(y)) => Ok((x, y)),
            _ => Err("Invalid input".to_string())
        }
    }).collect()
}



fn sum_corrupted_instructions(file_path: &str) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    file_contents.lines()
        .filter_map(|line| {
            match get_instructions_from_string(&line) {
                Ok(x) => Some(x.iter().map(|(x, y)| x * y).sum::<i32>()),
                Err(_) => None,
            }
        })
        .sum()
}

fn main() {
    let value = sum_corrupted_instructions("./test.txt");
    println!("Value: {}", value);
}

#[cfg(test)]
mod tests {
    use crate::{sum_corrupted_instructions};

    #[test]
    fn returns_expected_sum_for_corrupted_instructions_for_test_data() {
        let value = sum_corrupted_instructions("./test.txt");
        assert_eq!(value, 161);
    }

    #[test]
    fn returns_expected_sum_for_corrupted_instructions_for_input_data() {
        let value = sum_corrupted_instructions("./input.txt");
        assert_eq!(value, 156388521);
    }
}
