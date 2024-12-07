use std::collections::HashMap;
use std::fs;

fn get_is_calibrated(numbers: &Vec<i64>, target: i64) -> bool {
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
    ) -> bool {
        if pos == numbers.len() {
            return current == target;
        }

        if let Some(&result) = cache.get(&(pos, current)) {
            return result;
        }

        let num = numbers[pos];
        let result = calibrate(pos + 1, current + num, numbers, target, cache)
            || calibrate(pos + 1, current * num, numbers, target, cache);

        cache.insert((pos, current), result);
        result
    }

    calibrate(1, numbers[0], numbers, target, &mut cache)
}

fn get_total_calibration_result(file_path: &str) -> i64 {
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
        .filter_map(|(target, numbers)| get_is_calibrated(&numbers, target).then_some(target))
        .sum()
}

fn main() {
    println!(
        "Total calibration result: {}",
        get_total_calibration_result("./input.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::get_total_calibration_result;

    #[test]
    fn returns_expected_value_test_data() {
        let total_calibration_result = get_total_calibration_result("./test.txt");
        assert_eq!(total_calibration_result, 3749);
    }

    #[test]
    fn returns_expected_value_for_input_data() {
        let value = get_total_calibration_result("./input.txt");
        assert_eq!(value, 303876485655);
    }
}
