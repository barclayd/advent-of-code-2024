use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Default)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<u8>,
    instruction_pointer: usize,
}

impl Computer {
    fn parse_line(line: &str) -> i64 {
        line.split_once(": ")
            .map(|(_, val)| val.parse().unwrap_or_default())
            .unwrap_or_default()
    }

    fn from_input(input: &str) -> Computer {
        let mut lines = input.lines();

        Self {
            a: lines.next().map_or(0, Self::parse_line),
            b: lines.next().map_or(0, Self::parse_line),
            c: lines.next().map_or(0, Self::parse_line),
            program: lines
                .next()
                .and_then(|l| l.split_once(": "))
                .map(|(_, nums)| nums.split(',').filter_map(|num| num.parse().ok()).collect())
                .unwrap_or_default(),
            instruction_pointer: 0,
        }
    }

    fn step(&mut self) -> Option<i64> {
        let inst = self.program[self.instruction_pointer];
        let literal = self.program[self.instruction_pointer + 1] as i64;
        let combo = match literal {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => literal,
        };

        self.instruction_pointer += 2;

        let mut output = None;
        match inst {
            0 => self.a = self.a / (1 << combo),
            1 => self.b = self.b ^ literal,
            2 => self.b = combo % 8,
            3 if self.a != 0 => self.instruction_pointer = literal as usize,
            3 => {}
            4 => self.b = self.b ^ self.c,
            5 => output = Some(combo % 8),
            6 => self.b = self.a / (1 << combo),
            7 => self.c = self.a / (1 << combo),
            _ => panic!("Invalid instruction {inst}"),
        }

        output
    }

    fn run(&mut self) -> Vec<i64> {
        let mut result = Vec::new();

        while self.instruction_pointer < self.program.len() {
            if let Some(output) = self.step() {
                result.push(output);
            }
        }

        result
    }
}

fn format_result(result: Vec<i64>) -> String {
    let formatted_result = result.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");

    formatted_result
}

fn get_output_string(file_path: &str, part: Part) -> String {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut computer = Computer::from_input(&file_contents);

    if part == Part1 {
        let result = computer.run();
        format_result(result)
    } else {
        "2".to_string()
    }
}

fn main() {
    println!("Part 1 value: {}", get_output_string("./input.txt", Part1));
    println!("Part 2 value: {}", get_output_string("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::get_output_string;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_output_string("./test.txt", Part1);
        assert_eq!(value, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_output_string("./input.txt", Part1);
        assert_eq!(value, "4,0,4,7,1,2,7,1,6");
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_output_string("./test.txt", Part2);
        assert_eq!(value, "2");
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_output_string("./input.txt", Part2);
        assert_eq!(value, "2");
    }
}
