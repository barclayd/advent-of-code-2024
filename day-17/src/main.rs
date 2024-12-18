use std::fs;

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

    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.instruction_pointer = 0;
    }
    
    fn run_part_2(&mut self) -> Vec<i64> {
        let mut saved = Vec::new();

        for a in 1..1024 {
            self.reset();
            self.a = a;
            let output = self.run();
            if output[0] == self.program[0] as i64 {
                saved.push(a);
            }
        }

        let mut pos = 1;
        while pos < self.program.len() {
            let mut next = Vec::new();

            for consider in saved {
                for bit in 0..8 {
                    let num = (bit << (7 + 3 * pos)) | consider;
                    self.reset();
                    self.a = num;
                    let output = self.run();

                    if output.len() > pos && output[pos] == self.program[pos] as i64 {
                        next.push(num);
                    }
                }
            }
            pos += 1;

            saved = next;
        }
        
        saved
    }
}

fn format_result(result: Vec<i64>) -> String {
    let formatted_result = result
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");

    formatted_result
}

fn get_output_string(file_path: &str) -> String {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut computer = Computer::from_input(&file_contents);

    let result = computer.run();
    
    format_result(result)
}

fn get_lowest_positive_initial_value_for_register_a(file_path: &str) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let mut computer = Computer::from_input(&file_contents);
    
    let result = computer.run_part_2();

    result.iter().cloned().min().unwrap()
}

fn main() {
    println!("Output string: {}", get_output_string("./input.txt"));
    println!("Lowest positive initial value: {}", get_lowest_positive_initial_value_for_register_a("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::{get_output_string, get_lowest_positive_initial_value_for_register_a};

    #[test]
    fn returns_expected_output_string_for_test_data() {
        let value = get_output_string("./test.txt");
        assert_eq!(value, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn returns_expected_output_string_for_input_data() {
        let value = get_output_string("./input.txt");
        assert_eq!(value, "4,0,4,7,1,2,7,1,6");
    }

    #[test]
    fn returns_expected_lowest_positive_initial_value_test_data() {
        let value = get_lowest_positive_initial_value_for_register_a("./test-2.txt");
        assert_eq!(value, 117440);
    }

    #[test]
    fn returns_expected_lowest_positive_initial_value_for_input_data() {
        let value = get_lowest_positive_initial_value_for_register_a("./input.txt");
        assert_eq!(value, 202322348616234);
    }
}
