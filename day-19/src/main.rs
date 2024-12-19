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
    memo: HashMap<String, usize>,
}

impl Cache {
    fn count_valid_designs(&mut self, design: &str, patterns: &[String]) -> usize {
        if design.is_empty() {
            return 1;
        }
        
        if let Some(&count) = self.memo.get(design) {
            return count;
        }

        let count = patterns
            .iter()
            .filter(|pattern| design.starts_with(*pattern))
            .map(|pattern| self.count_valid_designs(&design[pattern.len()..], patterns))
            .sum();
            
        self.memo.insert(design.to_string(), count);
        count
    }
}

struct DesignParser {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl DesignParser {
    fn from_file(file_path: &str) -> std::io::Result<Self> {
        let contents = fs::read_to_string(file_path)?;
        let mut lines = contents.lines();
        
        let patterns = lines
            .next()
            .unwrap_or_default()
            .split(", ")
            .map(String::from)
            .collect();
            
        let designs = lines.skip(1)
            .map(String::from)
            .collect();
            
        Ok(Self { patterns, designs })
    }
    
    fn count_designs(&self, part: Part) -> usize {
        let mut cache = Cache::default();
        match part {
            Part1 => self.designs
                .iter()
                .filter(|design| cache.count_valid_designs(design, &self.patterns) > 0)
                .count(),
            Part2 => self.designs
                .iter()
                .map(|design| cache.count_valid_designs(design, &self.patterns))
                .sum()
        }
    }
}

fn get_value(file_path: &str, part: Part) -> usize {
    let parser = DesignParser::from_file(file_path)
        .expect("Failed to parse input file");
    
    parser.count_designs(part)
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
        assert_eq!(value, 16);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 662726441391898);
    }
}
