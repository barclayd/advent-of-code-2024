use std::collections::{HashSet, VecDeque};
use crate::Part::{Part1, Part2};
use std::fs;
use itertools::Itertools;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Invalid direction: {}", c),
        }
    }
}

impl Direction {
    fn to_delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Robot,
    Box,
    BoxLeft,
    BoxRight,
    Empty,
    Wall,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '@' => Tile::Robot,
            'O' => Tile::Box,
            '[' => Tile::BoxLeft,
            ']' => Tile::BoxRight,
            '.' => Tile::Empty,
            _ => Tile::Wall,
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Robot => '@',
            Tile::Box => 'O',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
            Tile::Empty => '.',
            Tile::Wall => '#',
        }
    }
}

struct Warehouse {
    grid: Vec<Vec<Tile>>,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();
        Self { grid }
    }

    fn find_robot(&self) -> Option<(i32, i32)> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == Tile::Robot {
                    return Some((y as i32, x as i32));
                }
            }
        }
        None
    }

    fn get_tile(&self, y: i32, x: i32) -> Tile {
        self.grid
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(Tile::Wall)
    }

    fn set_tile(&mut self, y: i32, x: i32, tile: Tile) {
        if let (Some(row), Some(col)) = (self.grid.get_mut(y as usize), Some(x as usize)) {
            if let Some(cell) = row.get_mut(col) {
                *cell = tile;
            }
        }
    }

    fn swap_tiles(&mut self, y1: i32, x1: i32, y2: i32, x2: i32) {
        let tile1 = self.get_tile(y1, x1);
        let tile2 = self.get_tile(y2, x2);
        self.set_tile(y1, x1, tile2);
        self.set_tile(y2, x2, tile1);
    }

    fn attempt_move(&mut self, direction: Direction) {
        let (y, x) = self.find_robot().expect("Robot not found");
        let (dy, dx) = direction.to_delta();
        let (mut y2, mut x2) = (y + dy, x + dx);

        match self.get_tile(y2, x2) {
            Tile::Empty => self.swap_tiles(y, x, y2, x2),
            Tile::Box => {
                let mut boxes_to_move = vec![];
                while self.get_tile(y2, x2) == Tile::Box {
                    boxes_to_move.push((y2, x2));
                    y2 += dy;
                    x2 += dx;
                }

                if self.get_tile(y2, x2) == Tile::Empty {
                    for &(by, bx) in boxes_to_move.iter().rev() {
                        self.swap_tiles(by, bx, by + dy, bx + dx);
                    }
                    self.swap_tiles(y, x, y + dy, x + dx);
                }
            }
            _ => {}
        }
    }

    fn from_scaled(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .flat_map(|c| match c {
                        '#' => vec![Tile::Wall, Tile::Wall],
                        'O' => vec![Tile::BoxLeft, Tile::BoxRight],
                        '.' => vec![Tile::Empty, Tile::Empty],
                        '@' => vec![Tile::Robot, Tile::Empty],
                        _ => vec![Tile::from(c)],
                    })
                    .collect()
            })
            .collect();
        Self { grid }
    }

    fn attempt_scaled_move(&mut self, direction: Direction) {
        if let Some((y, x)) = self.find_robot() {
            let (dy, dx) = direction.to_delta();
            let (y2, x2) = (y + dy, x + dx);

            match self.get_tile(y2, x2) {
                Tile::Empty => self.swap_tiles(y, x, y2, x2),
                Tile::BoxLeft | Tile::BoxRight => {
                    let mut queue = VecDeque::from([(y, x)]);
                    let mut seen = HashSet::new();
                    
                    while let Some((cy, cx)) = queue.pop_front() {
                        if seen.insert((cy, cx)) {
                            let ny = cy + dy;
                            let nx = cx + dx;
                            match self.get_tile(ny, nx) {
                                Tile::Wall => return,
                                Tile::BoxLeft => {
                                    queue.extend([(ny, nx), (ny, nx + 1)]);
                                }
                                Tile::BoxRight => {
                                    queue.extend([(ny, nx), (ny, nx - 1)]);
                                }
                                _ => continue,
                            }
                        }
                    }

                    let seen_sorted = match (dy, dx) {
                        (-1, 0) => seen.iter().sorted_by_key(|&&(y, _)| y),
                        (0, 1) => seen.iter().sorted_by_key(|&&(_, x)| -x),
                        (1, 0) => seen.iter().sorted_by_key(|&&(y, _)| -y),
                        _ => seen.iter().sorted_by_key(|&&(_, x)| x),
                    };

                    for &(sy, sx) in seen_sorted {
                        self.swap_tiles(sy + dy, sx + dx, sy, sx);
                    }
                }
                _ => {}
            }
        }
    }

    fn calculate_gps_score(&self, scoring_tile: Tile) -> usize {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &tile)| {
                    if tile == scoring_tile {
                        Some(100 * y + x)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }
}

fn get_value(file_path: &str, part: Part) -> usize {
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let (map_str, moves_str) = file_contents
        .split_once("\n\n")
        .expect("Invalid input format");

    let moves = moves_str
        .chars()
        .filter(|c| *c != '\n')
        .map(Direction::from);

    match part {
        Part1 => {
            let mut warehouse = Warehouse::new(map_str);
            for direction in moves {
                warehouse.attempt_move(direction);
            }
            warehouse.calculate_gps_score(Tile::Box)
        }
        Part2 => {
            let mut warehouse = Warehouse::from_scaled(map_str);
            for direction in moves {
                warehouse.attempt_scaled_move(direction);
            }
            warehouse.calculate_gps_score(Tile::BoxLeft)
        }
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
        assert_eq!(value, 10092);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 1465523);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 9021);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 1471049);
    }
}
