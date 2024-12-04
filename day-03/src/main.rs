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

fn get_instructions_with_conditional_statements_from_string(
    input: &str,
    initial_instructions_enabled: bool
) -> Result<(Vec<i32>, bool), String> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();
    
    let results = re.captures_iter(input)
        .try_fold((Vec::new(), initial_instructions_enabled), |(mut results, instructions_enabled), caps| {
            let instruction = caps.get(0).unwrap().as_str();
            
            match instruction {
                s if s.starts_with("mul") => {
                    let (first, second) = (caps.get(1).unwrap(), caps.get(2).unwrap());
                    match (first.as_str().parse::<i32>(), second.as_str().parse::<i32>()) {
                        (Ok(x), Ok(y)) if instructions_enabled => {
                            results.push(x * y);
                            Ok((results, instructions_enabled))
                        },
                        (Ok(_), Ok(_)) => Ok((results, instructions_enabled)),
                        _ => Err("Invalid input".to_string())
                    }
                },
                "don't()" => Ok((results, false)),
                "do()" => Ok((results, true)),
                _ => Err("Invalid input".to_string())
            }
        });

    results
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

fn sum_corrupted_instructions_with_conditional_statements(file_path: &str) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut final_sum = 0;
    let mut instructions_enabled = true;

    for line in file_contents.lines() {
        match get_instructions_with_conditional_statements_from_string(line, instructions_enabled) {
            Ok((results, final_instructions_enabled)) => {
                final_sum += results.iter().sum::<i32>();
                instructions_enabled = final_instructions_enabled;
            }
            Err(_) => continue,
        }
    }

    final_sum
}

fn main() {
    let value = sum_corrupted_instructions("./test.txt");
    println!("Value: {}", value);
    let value_from_corrupted_instructions = sum_corrupted_instructions_with_conditional_statements("./test.txt");
    println!("Value: {}", value_from_corrupted_instructions);
}

#[cfg(test)]
mod tests {
    use crate::{sum_corrupted_instructions, sum_corrupted_instructions_with_conditional_statements};

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

    #[test]
    fn returns_expected_sum_for_corrupted_instructions_with_conditional_statements_for_test_data() {
        let value = sum_corrupted_instructions_with_conditional_statements("./test-2.txt");
        assert_eq!(value, 48);
    }

    #[test]
    fn returns_expected_sum_for_corrupted_instructions_with_conditional_statements_for_input_data() {
        let value = sum_corrupted_instructions_with_conditional_statements("./input.txt");
        assert_eq!(value, 75920122);
    }
}
