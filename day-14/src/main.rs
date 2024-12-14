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

fn get_challenge_value(file_path: &str, part: Part) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let robots: Vec<Robot> = file_contents
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    if part == Part1 {
        get_safety_factor(robots)
    } else {
        get_fewest_seconds_to_form_picture_of_christmas_tree(robots)
    }
}

fn get_safety_factor(mut robots: Vec<Robot>) -> i32 {
    for _ in 1..=100 {
        advance_robots(&mut robots);
    }

    calculate_safety_factor(&robots)
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
        if robot.position.x == mid_x || robot.position.y == mid_y {
            continue;
        }

        match (robot.position.x > mid_x, robot.position.y > mid_y) {
            (true, true) => q1 += 1,
            (false, true) => q2 += 1,
            (false, false) => q3 += 1,
            (true, false) => q4 += 1,
        }
    }

    q1 * q2 * q3 * q4
}

fn get_fewest_seconds_to_form_picture_of_christmas_tree(mut robots: Vec<Robot>) -> i32 {
    let mut seconds_to_form_picture_of_christmas_tree = 0;
    
    for seconds in 1.. {
        advance_robots(&mut robots);

        let positions: Vec<_> = robots.iter()
            .map(|robot| (robot.position.x, robot.position.y))
            .collect();

        if positions.len() == positions.iter().collect::<std::collections::HashSet<_>>().len() {
            seconds_to_form_picture_of_christmas_tree = seconds;
            break;
        }
    }
    
    seconds_to_form_picture_of_christmas_tree
}

fn main() {
    println!("Part 1 value: {}", get_challenge_value("./input.txt", Part1));
    println!("Part 2 value: {}", get_challenge_value("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::get_challenge_value;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_challenge_value("./test.txt", Part1);
        assert_eq!(value, 21);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_challenge_value("./input.txt", Part1);
        assert_eq!(value, 225648864);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_challenge_value("./input.txt", Part2);
        assert_eq!(value, 7847);
    }
}
