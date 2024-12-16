use crate::Part::{Part1, Part2};
use std::fs;

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
enum GridItem {
    Robot,
    Box,
    Empty,
    Wall,
}

impl From<char> for GridItem {
    fn from(c: char) -> Self {
        match c {
            '@' => GridItem::Robot,
            'O' => GridItem::Box,
            '.' => GridItem::Empty,
            _ => GridItem::Wall,
        }
    }
}

impl Into<char> for GridItem {
    fn into(self) -> char {
        match self {
            GridItem::Robot => '@',
            GridItem::Box => 'O',
            GridItem::Empty => '.',
            GridItem::Wall => '#',
        }
    }
}

struct Warehouse {
    grid: Vec<Vec<GridItem>>,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| line.chars().map(GridItem::from).collect())
            .collect();
        Self { grid }
    }

    fn find_robot(&self) -> Option<(i32, i32)> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile == GridItem::Robot {
                    return Some((y as i32, x as i32));
                }
            }
        }
        None
    }

    fn get_tile(&self, y: i32, x: i32) -> GridItem {
        self.grid
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
            .copied()
            .unwrap_or(GridItem::Wall)
    }

    fn set_tile(&mut self, y: i32, x: i32, tile: GridItem) {
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
            GridItem::Empty => self.swap_tiles(y, x, y2, x2),
            GridItem::Box => {
                let mut boxes_to_move = vec![];
                while self.get_tile(y2, x2) == GridItem::Box {
                    boxes_to_move.push((y2, x2));
                    y2 += dy;
                    x2 += dx;
                }

                if self.get_tile(y2, x2) == GridItem::Empty {
                    for &(by, bx) in boxes_to_move.iter().rev() {
                        self.swap_tiles(by, bx, by + dy, bx + dx);
                    }
                    self.swap_tiles(y, x, y + dy, x + dx);
                }
            }
            _ => {}
        }
    }

    fn calculate_gps_score(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &tile)| {
                    if tile == GridItem::Box {
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
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (map_str, moves_str) = file_contents
        .split_once("\n\n")
        .expect("Invalid input format");

    let mut warehouse = Warehouse::new(map_str);

    if part == Part1 {
        let moves = moves_str
            .chars()
            .filter(|c| *c != '\n')
            .map(Direction::from);

        for direction in moves {
            warehouse.attempt_move(direction);
        }

        warehouse.calculate_gps_score()
    } else {
        4
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
        assert_eq!(value, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}
