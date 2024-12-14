use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use crate::Part::{Part1, Part2};

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn advance(&mut self, velocity: &Coordinates, width: i32, height: i32) {
        self.x = (self.x + velocity.x).rem_euclid(width);
        self.y = (self.y + velocity.y).rem_euclid(height);
    }
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    position: Coordinates,
    velocity: Coordinates,
}

impl Robot {
    fn advance(&mut self, width: i32, height: i32) {
        self.position.advance(&self.velocity, width, height);
    }

    fn is_on_grid_line(&self, mid_x: i32, mid_y: i32) -> bool {
        self.position.x == mid_x || self.position.y == mid_y
    }

    fn get_quadrant(&self, mid_x: i32, mid_y: i32) -> Option<Quadrant> {
        if self.is_on_grid_line(mid_x, mid_y) {
            return None;
        }

        Some(match (self.position.x > mid_x, self.position.y > mid_y) {
            (true, true) => Quadrant::TopRight,
            (false, true) => Quadrant::TopLeft,
            (false, false) => Quadrant::BottomLeft,
            (true, false) => Quadrant::BottomRight,
        })
    }
}

#[derive(Debug)]
enum Quadrant {
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}

struct Grid {
    width: i32,
    height: i32,
}

impl Grid {
    fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    fn mid_x(&self) -> i32 { self.width / 2 }
    fn mid_y(&self) -> i32 { self.height / 2 }
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y] = s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<i32>, _>>()
            .map_err(|_| "Failed to parse coordinates")?[..] else {
                return Err("Invalid coordinate format".to_string());
            };

        Ok(Self::new(x, y))
    }
}

impl FromStr for Robot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [pos, vel] = s.split_whitespace().collect::<Vec<_>>()[..] else {
            return Err("Invalid robot format".to_string());
        };

        Ok(Self {
            position: pos.trim_start_matches("p=").parse()?,
            velocity: vel.trim_start_matches("v=").parse()?,
        })
    }
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn get_challenge_value(file_path: &str, part: Part) -> i32 {
    let robots = parse_input(file_path);

    match part {
        Part1 => get_safety_factor(robots),
        Part2 => get_fewest_seconds_to_form_picture(robots),
    }
}

fn parse_input(file_path: &str) -> Vec<Robot> {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn get_safety_factor(mut robots: Vec<Robot>) -> i32 {
    let grid = Grid::new(WIDTH, HEIGHT);
    
    for _ in 1..=100 {
        advance_robots(&mut robots, &grid);
    }

    calculate_safety_factor(&robots, &grid)
}

fn advance_robots(robots: &mut [Robot], grid: &Grid) {
    robots.iter_mut()
        .for_each(|robot| robot.advance(grid.width, grid.height));
}

fn calculate_safety_factor(robots: &[Robot], grid: &Grid) -> i32 {
    let mut quadrant_counts = [0; 4];

    for robot in robots {
        if let Some(quadrant) = robot.get_quadrant(grid.mid_x(), grid.mid_y()) {
            quadrant_counts[quadrant as usize] += 1;
        }
    }

    quadrant_counts.iter().product()
}

fn get_fewest_seconds_to_form_picture(mut robots: Vec<Robot>) -> i32 {
    let grid = Grid::new(WIDTH, HEIGHT);
    
    for seconds in 1.. {
        advance_robots(&mut robots, &grid);

        let unique_positions: HashSet<_> = robots.iter()
            .map(|robot| robot.position)
            .collect();

        if unique_positions.len() == robots.len() {
            return seconds;
        }
    }
    
    unreachable!("Solution should be found")
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
