use crate::Part::{Part1, Part2};
use std::fs;
use std::str::FromStr;

const PART_2_SCALE: i64 = 10_000_000_000_000;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Copy, Clone)]
struct Claw {
    button_a: Position,
    button_b: Position,
    prize: Position,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn offset(&self, value: i64) -> Self {
        Self {
            x: self.x + value,
            y: self.y + value,
        }
    }

    fn as_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

impl Claw {
    fn with_inflated_prize(&self, offset: i64) -> Self {
        Self {
            prize: self.prize.offset(offset),
            ..*self
        }
    }

    fn calculate_tokens(&self) -> Option<i64> {
        let det = self.button_a.x * self.button_b.y - self.button_a.y * self.button_b.x;

        if det == 0 {
            return None;
        }

        let n = (self.button_a.x * self.prize.y - self.button_a.y * self.prize.x) / det;
        let m = (self.prize.x - self.button_b.x * n) / self.button_a.x;

        let calculated_position = Position::new(
            self.button_a.x * m + self.button_b.x * n,
            self.button_a.y * m + self.button_b.y * n,
        );

        (calculated_position.as_tuple() == self.prize.as_tuple()).then_some(3 * m + n)
    }
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(", ").collect();
        let [px, py] = parts[..2] else {
            return Err("Invalid coordinate format".to_string());
        };

        Ok(Self::new(
            parse_coordinate(px, "X")?,
            parse_coordinate(py, "Y")?,
        ))
    }
}

impl FromStr for Claw {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [a, b, p] = s.lines().collect::<Vec<_>>()[..3] else {
            return Err("Invalid number of lines".to_string());
        };

        Ok(Self {
            button_a: parse_value(a)?,
            button_b: parse_value(b)?,
            prize: parse_value(p)?,
        })
    }
}

fn parse_coordinate(input: &str, prefix: &str) -> Result<i64, String> {
    input
        .trim_start_matches(&format!("{}+", prefix))
        .trim_start_matches(&format!("{}=", prefix))
        .parse()
        .map_err(|_| format!("Failed to parse {} coordinate", prefix))
}

fn parse_value(line: &str) -> Result<Position, String> {
    line.split(": ")
        .nth(1)
        .ok_or_else(|| "Invalid format".to_string())?
        .parse()
}

fn parse_input(input: &str) -> Result<Vec<Claw>, String> {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

fn calculate_total_tokens(claws: &[Claw], part: Part) -> i64 {
    let scale = if part == Part2 { PART_2_SCALE } else { 0 };

    claws
        .iter()
        .map(|claw| claw.with_inflated_prize(scale))
        .filter_map(|claw| claw.calculate_tokens())
        .sum()
}

fn get_minimum_amount_of_tokens_spent_to_win_all_prizes(file_path: &str, part: Part) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let claws = parse_input(&file_contents).expect("Failed to parse input");

    calculate_total_tokens(&claws, part)
}

fn main() {
    println!(
        "Part 1 value: {}",
        get_minimum_amount_of_tokens_spent_to_win_all_prizes("./input.txt", Part1)
    );
    println!(
        "Part 2 value: {}",
        get_minimum_amount_of_tokens_spent_to_win_all_prizes("./test.txt", Part2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_minimum_amount_of_tokens_spent_to_win_all_prizes;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let minimum_amount_of_tokens_spent_to_win_prizes =
            get_minimum_amount_of_tokens_spent_to_win_all_prizes("./test.txt", Part1);
        assert_eq!(minimum_amount_of_tokens_spent_to_win_prizes, 480);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let minimum_amount_of_tokens_spent_to_win_prizes =
            get_minimum_amount_of_tokens_spent_to_win_all_prizes("./input.txt", Part1);
        assert_eq!(minimum_amount_of_tokens_spent_to_win_prizes, 40369);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let minimum_amount_of_tokens_spent_to_win_prizes =
            get_minimum_amount_of_tokens_spent_to_win_all_prizes("./test.txt", Part2);
        assert_eq!(minimum_amount_of_tokens_spent_to_win_prizes, 875318608908);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let minimum_amount_of_tokens_spent_to_win_prizes =
            get_minimum_amount_of_tokens_spent_to_win_all_prizes("./input.txt", Part2);
        assert_eq!(minimum_amount_of_tokens_spent_to_win_prizes, 72587986598368);
    }
}
