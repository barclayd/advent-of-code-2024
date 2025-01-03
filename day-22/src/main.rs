use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

use std::collections::{HashMap, HashSet, VecDeque};

const PRUNE: isize = 16777216;

fn next_number(previous: isize) -> isize {
    let mut step_1 = previous * 64;
    step_1 ^= previous;
    step_1 %= PRUNE;

    let mut step_2 = step_1 / 32;
    step_2 ^= step_1;
    step_2 %= PRUNE;

    let mut final_step = step_2 * 2_048;
    final_step ^= step_2;

    final_step % PRUNE
}

struct SecretProcessor {
    secrets: Vec<isize>,
}

impl SecretProcessor {
    fn from_file(file_path: &str) -> Self {
        let secrets = fs::read_to_string(file_path)
            .expect("Should have been able to read the file")
            .trim()
            .lines()
            .map(|elem| elem.parse())
            .collect::<Result<Vec<isize>, _>>()
            .unwrap();
            
        Self { secrets }
    }

    fn process_part1(&self) -> isize {
        let mut total = 0;
        for &secret in &self.secrets {
            let mut temp = secret;
            for _ in 0..2_000 {
                temp = next_number(temp);
            }
            total += temp;
        }
        total
    }

    fn process_part2(&self) -> isize {
        let all_secrets: Vec<Vec<isize>> = self.secrets
            .iter()
            .map(|&code| {
                let mut secrets = vec![code];
                for _ in 0..2_000 {
                    secrets.push(next_number(*secrets.last().unwrap()));
                }
                secrets.into_iter().map(|elem| elem % 10).collect()
            })
            .collect();

        let mut changes_values: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();

        for secrets in all_secrets {
            let mut changes: VecDeque<isize> = VecDeque::new();
            let mut already_changed: HashSet<(isize, isize, isize, isize)> = HashSet::new();
            for (i, code) in secrets.iter().enumerate().skip(1) {
                changes.push_back(*code - secrets[i - 1]);

                if changes.len() == 4 {
                    let variation = (changes[0], changes[1], changes[2], changes[3]);
                    if already_changed.insert(variation) {
                        let total = changes_values.entry(variation).or_default();
                        *total += code;
                    }
                    changes.pop_front();
                }
            }
        }

        let (_, total) = changes_values
            .iter()
            .max_by(|(_, &total_a), (_, total_b)| total_a.cmp(total_b))
            .unwrap();

        *total
    }
}

fn get_value(file_path: &str, part: Part) -> isize {
    let processor = SecretProcessor::from_file(file_path);
    match part {
        Part1 => processor.process_part1(),
        Part2 => processor.process_part2(),
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
        assert_eq!(value, 94558292);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 19241711734);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 90);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 2058);
    }
}
