use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn get_distinct_positions(file_path: &str) -> Vec<(usize, usize)> {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let initial_map: Vec<Vec<char>> = file_contents.lines().map(|line| line.chars().collect()).collect();
    
    simulate(&initial_map)
}

fn simulate(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let (mut y, mut x) = locate_guard(map);
    let mut direction = Direction::Up;
    let mut distinct_positions = vec![vec![[false; 4]; map[0].len()]; map.len()];

    loop {
        distinct_positions[y as usize][x as usize][direction as usize] = true;
        let (dy, dx) = direction.delta();
        
        match get_next_position(map, y + dy, x + dx) {
            ' ' => {
                return get_visited_positions(map, &distinct_positions);
            }
            '#' => direction = direction.turn_right(),
            _ => {
                y += dy;
                x += dx;
            }
        }
    }
}

fn locate_guard(map: &Vec<Vec<char>>) -> (i32, i32) {
    let mut pos: (i32, i32) = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                pos = (y as i32, x as i32);
            }
        }
    }
    pos
}

fn get_next_position(chars: &Vec<Vec<char>>, y: i32, x: i32) -> char {
    *chars
        .get(y as usize)
        .and_then(|c| c.get(x as usize))
        .unwrap_or(&' ')
}

fn get_visited_positions(map: &Vec<Vec<char>>, distinct_positions: &Vec<Vec<[bool; 4]>>) -> Vec<(usize, usize)> {
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|&(y, x)| distinct_positions[y][x].iter().any(|&dir_seen| dir_seen))
        .collect()
}

fn main() {
    let distinct_positions_count = get_distinct_positions("./input.txt").len();
    println!("Distinct positions: {}", distinct_positions_count);
}

#[cfg(test)]
mod tests {
    use crate::get_distinct_positions;

    #[test]
    fn returns_expected_distinct_positions_count_for_test_data() {
        let distinct_positions_count = get_distinct_positions("./test.txt").len();
        assert_eq!(distinct_positions_count, 41);
    }

    #[test]
    fn returns_expected_distinct_positions_count_for_input_data() {
        let distinct_positions_count = get_distinct_positions("./input.txt").len();
        assert_eq!(distinct_positions_count, 4967);
    }
}