use std::collections::{HashSet, VecDeque};
use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [Direction::North, Direction::East, Direction::South, Direction::West]
    }

    fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Eq, Hash)]
struct Position {
    y: i32,
    x: i32,
}

impl Position {
    fn new(y: usize, x: usize) -> Self {
        Self {
            y: y as i32,
            x: x as i32,
        }
    }

    fn move_in_direction(&self, direction: Direction) -> Self {
        let (dy, dx) = direction.to_offset();
        Self {
            y: self.y + dy,
            x: self.x + dx,
        }
    }

    fn to_usize(&self) -> Option<(usize, usize)> {
        if self.y >= 0 && self.x >= 0 {
            Some((self.y as usize, self.x as usize))
        } else {
            None
        }
    }
}

fn get_value_at_position(map: &[Vec<char>], pos: Position) -> Option<u32> {
    pos.to_usize()
        .and_then(|(y, x)| {
            map.get(y)
                .and_then(|row| row.get(x))
                .and_then(|&c| c.to_digit(10))
        })
}

fn get_trailheads_score(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let map: Vec<Vec<char>> = file_contents.lines().map(|l| l.chars().collect()).collect();

    if part == Part1 {
        get_trailheads(&map)
            .iter()
            .map(|(y, x)| calculate_score(&map, (*y, *x)))
            .sum()
    } else {
        4
    }
}

fn get_trailheads(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '0' {
                trailheads.push((y, x));
            }
        }
    }
    trailheads
}

fn calculate_score(map: &[Vec<char>], start: (usize, usize)) -> usize {
    let mut score = 0;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    
    queue.push_back(Position::new(start.0, start.1));
    
    while let Some(current_pos) = queue.pop_front() {
        if !seen.insert(current_pos) {
            continue;
        }

        if let Some((y, x)) = current_pos.to_usize() {
            if map[y][x] == '9' {
                score += 1;
            }
        }

        let current_value = get_value_at_position(map, current_pos)
            .expect("Invalid position");

        for direction in Direction::all() {
            let next_pos = current_pos.move_in_direction(direction);
            
            if let Some(next_value) = get_value_at_position(map, next_pos) {
                if current_value + 1 == next_value {
                    queue.push_back(next_pos);
                }
            }
        }
    }
    
    score
}

fn main() {
    println!(
        "Part 1 value: {}",
        get_trailheads_score("./input.txt", Part1)
    );
    println!(
        "Part 2 value: {}",
        get_trailheads_score("./input.txt", Part2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_trailheads_score;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let trailheads_score = get_trailheads_score("./test.txt", Part1);
        assert_eq!(trailheads_score, 36);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let trailheads_score = get_trailheads_score("./input.txt", Part1);
        assert_eq!(trailheads_score, 816);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let trailheads_score = get_trailheads_score("./test.txt", Part2);
        assert_eq!(trailheads_score, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let trailheads_score = get_trailheads_score("./input.txt", Part2);
        assert_eq!(trailheads_score, 4);
    }
}
