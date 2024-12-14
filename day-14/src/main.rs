use crate::Part::{Part1, Part2};
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err("Invalid coordinate format".to_string());
        }

        let x = parts[0]
            .parse()
            .map_err(|_| "Failed to parse x coordinate")?;
        let y = parts[1]
            .parse()
            .map_err(|_| "Failed to parse y coordinate")?;

        Ok(Coordinates { x, y })
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Coordinates,
    velocity: Coordinates,
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err("Invalid robot format".to_string());
        }

        let position = parts[0].trim_start_matches("p=").parse()?;
        let velocity = parts[1].trim_start_matches("v=").parse()?;

        Ok(Robot { position, velocity })
    }
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn get_safety_factor(file_path: &str, part: Part) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut robots: Vec<Robot> = file_contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();
    
    for _ in 1..=100 {
        advance_robots(&mut robots);
    }

    if part == Part1 {
        calculate_safety_factor(&robots)
    } else {
        4
    }
}

fn advance_robots(robots: &mut Vec<Robot>) {
    for robot in robots {
        robot.position.x = (robot.position.x + robot.velocity.x).rem_euclid(WIDTH);
        robot.position.y = (robot.position.y + robot.velocity.y).rem_euclid(HEIGHT);
    }
}

fn calculate_safety_factor(robots: &Vec<Robot>) -> i32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    for robot in robots {
        // Skip robots on the middle lines
        if robot.position.x == mid_x || robot.position.y == mid_y {
            continue;
        }

        match (robot.position.x > mid_x, robot.position.y > mid_y) {
            (true, true) => q1 += 1,   // Top Right
            (false, true) => q2 += 1,  // Top Left
            (false, false) => q3 += 1, // Bottom Left
            (true, false) => q4 += 1,  // Bottom Right
        }
    }
    
    // Debug print to see quadrant counts
    println!("Q1: {}, Q2: {}, Q3: {}, Q4: {}", q1, q2, q3, q4);
    
    q1 * q2 * q3 * q4
}

fn main() {
    println!("Part 1 value: {}", get_safety_factor("./input.txt", Part1));
    println!("Part 2 value: {}", get_safety_factor("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::get_safety_factor;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_safety_factor("./test.txt", Part1);
        assert_eq!(value, 12);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_safety_factor("./input.txt", Part1);
        assert_eq!(value, 8);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_safety_factor("./test.txt", Part2);
        assert_eq!(value, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_safety_factor("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}
