use std::collections::HashMap;
use std::fs;
use crate::Part::{Part1, Part2};

#[derive(PartialEq, Debug, Clone)]
enum Part {
    Part1,
    Part2,
}

fn concatenation_operator(first_number: i64, second_number: i64) -> i64 {
    let joined_number = format!("{}{}", first_number, second_number)
    .parse()
        .expect("Should have been able to parse the joined number");
    joined_number
}

fn get_is_calibrated(numbers: &Vec<i64>, target: i64, part: &Part) -> bool {
    if numbers.len() == 1 {
        return numbers[0] == target;
    }

    let mut cache: HashMap<(usize, i64), bool> = HashMap::new();

    fn calibrate(
        pos: usize,
        current: i64,
        numbers: &[i64],
        target: i64,
        cache: &mut HashMap<(usize, i64), bool>,
        part: &Part,
    ) -> bool {
        if pos == numbers.len() {
            return current == target;
        }

        if let Some(&result) = cache.get(&(pos, current)) {
            return result;
        }

        let num = numbers[pos];
        let result;
        
        if part == &Part1 {
            result =  calibrate(pos + 1, current + num, numbers, target, cache, part)
                || calibrate(pos + 1, current * num, numbers, target, cache, part);
        } else {
            result = calibrate(pos + 1, current + num, numbers, target, cache, part)
                || calibrate(pos + 1, current * num, numbers, target, cache, part)
                || calibrate(pos + 1, concatenation_operator(current, num), numbers, target, cache, part);
        }

        cache.insert((pos, current), result);
        result
    }

    calibrate(1, numbers[0], numbers, target, &mut cache, part)
}

fn get_total_calibration_result(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let parsed_lines: Vec<(i64, Vec<i64>)> = file_contents
        .lines()
        .filter_map(|line| {
            let (test_value, numbers) = line.split_once(":")?;
            Some((
                test_value.parse().ok()?,
                numbers
                    .split_whitespace()
                    .map(|n| n.parse().ok())
                    .collect::<Option<Vec<i64>>>()?,
            ))
        })
        .collect();

    parsed_lines
        .into_iter()
        .filter_map(|(target, numbers)| get_is_calibrated(&numbers, target, &part).then_some(target))
        .sum()
}

fn main() {
    println!(
        "Total calibration result for part 1: {}",
        get_total_calibration_result("./input.txt", Part1)
    );
    println!(
        "Total calibration result for part 2: {}",
        get_total_calibration_result("./input.txt", Part2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_total_calibration_result;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let total_calibration_result = get_total_calibration_result("./test.txt", Part1);
        assert_eq!(total_calibration_result, 3749);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let total_calibration_result = get_total_calibration_result("./input.txt", Part1);
        assert_eq!(total_calibration_result, 303876485655);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let total_calibration_result = get_total_calibration_result("./test.txt", Part2);
        assert_eq!(total_calibration_result, 11387);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let total_calibration_result = get_total_calibration_result("./input.txt", Part2);
        assert_eq!(total_calibration_result, 146111650210682);
    }
}
