use crate::Part::{Part1, Part2};
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Default)]
struct Cache {
    cache: HashMap<String, usize>,
}

impl Cache {
    fn can_make_design(&mut self, design: &str, patterns: &[String]) -> usize {
        if design.is_empty() {
            return 1;
        }
        if let Some(design_count) = self.cache.get(design) {
            return *design_count;
        }

        let count = patterns
            .iter()
            .filter(|pattern| design.starts_with(*pattern))
            .map(|pattern| self.can_make_design(&design[pattern.len()..], patterns))
            .sum();
        self.cache.insert(design.to_string(), count);
        count
    }
}

fn get_value(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines = file_contents.lines();
    
    let patterns: Vec<String> = lines
        .next()
        .unwrap_or_default()
        .split(", ")
        .map(String::from)
        .collect();
        
    let designs = lines.map(String::from).skip(1).collect::<Vec<String>>();
    
    println!("{:?}", patterns);
    println!("{:?}", designs);
    

    if part == Part1 {
        let mut cache = Cache::default();
        designs.iter().filter(|design| cache.can_make_design(design, &*patterns) > 0).count()
    } else {
        4
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
        assert_eq!(value, 6);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 308);
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
