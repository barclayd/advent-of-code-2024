use crate::Part::{Part1, Part2};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
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

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    y: i32,
    x: i32,
    direction: StateDirection,
    cost: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StateDirection {
    Enum(Direction),
    Index(i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost.cmp(&self.cost))
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.x.cmp(&other.x))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    grid: Vec<Vec<char>>,
}

impl Maze {
    fn new(input: &str) -> Self {
        let grid = input.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    fn find_char(&self, c: char) -> Option<(i32, i32)> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &char) in row.iter().enumerate() {
                if char == c {
                    return Some((y as i32, x as i32));
                }
            }
        }
        None
    }

    fn find_shortest_path_score(&self) -> u32 {
        let (start, end) = self.get_start_and_end();
        
        let mut shortest_path_score = u32::MAX;
        let mut heap = BinaryHeap::from([State {
            y: start.0,
            x: start.1,
            direction: StateDirection::Enum(Direction::Right),
            cost: 0,
        }]);
        let mut seen = HashSet::new();
        let mut dist = HashMap::new();

        while let Some(State { y, x, direction, cost }) = heap.pop() {
            if let StateDirection::Enum(dir) = direction {
                if !dist.contains_key(&(y, x, dir)) {
                    *dist.entry((y, x, dir)).or_default() = cost;
                }
                if (y, x) == end && cost < shortest_path_score {
                    shortest_path_score = cost;
                }
                if seen.insert((y, x, dir)) {
                    let (dy, dx) = dir.to_delta();
                    let (y2, x2) = (y + dy, x + dx);

                    if self.is_within_bounds(y2, x2) {
                        if self.get_char_at(y2, x2) != &'#' {
                            heap.push(State {
                                y: y2,
                                x: x2,
                                direction: StateDirection::Enum(dir),
                                cost: cost + 1,
                            });
                        }
                        
                        heap.push(State {
                            y,
                            x,
                            direction: StateDirection::Enum(dir.turn_right()),
                            cost: cost + 1000,
                        });
                        
                        heap.push(State {
                            y,
                            x,
                            direction: StateDirection::Enum(dir.turn_left()),
                            cost: cost + 1000,
                        });
                    }
                }
            }
        }
        shortest_path_score
    }

    fn get_number_of_tiles_on_shortest_paths(&self) -> usize {
        let (start, end) = self.get_start_and_end();

        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut dist = HashMap::new();
        let init_heap1 = vec![State {
            y: start.0,
            x: start.1,
            direction: StateDirection::Index(1),
            cost: 0,
        }];
        let best = self.find_shortest_path_part_2(end, dirs, init_heap1, &mut dist);

        let dirs2 = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        let mut dist2 = HashMap::new();
        let init_heap2 = (0..dirs2.len())
            .map(|dir| State {
                y: end.0,
                x: end.1,
                direction: StateDirection::Index(dir as i32),
                cost: 0,
            })
            .collect();
        self.find_shortest_path_part_2(start, dirs2, init_heap2, &mut dist2);

        let mut tiles = HashSet::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                for dir in 0..dirs.len() {
                    let tile = (y as i32, x as i32, dir as i32);
                    if dist.get(&tile).unwrap_or(&0) + dist2.get(&tile).unwrap_or(&0) == best {
                        tiles.insert((y, x));
                    }
                }
            }
        }
        tiles.len()
    }

    fn find_shortest_path_part_2(
        &self,
        end: (i32, i32),
        dirs: [(i32, i32); 4],
        initial_heap: Vec<State>,
        dist: &mut HashMap<(i32, i32, i32), u32>,
    ) -> u32 {
        let mut best = u32::MAX;
        let mut heap = BinaryHeap::from(initial_heap);
        let mut seen = HashSet::from([]);
        while let Some(State { y, x, direction, cost }) = heap.pop() {
            if let StateDirection::Index(dir) = direction {
                if !dist.contains_key(&(y, x, dir)) {
                    *dist.entry((y, x, dir)).or_default() = cost;
                }
                if (y, x) == end && cost < best {
                    best = cost
                }
                if seen.insert((y, x, dir)) {
                    let (dy, dx) = dirs[dir as usize];
                    let (y2, x2) = (y + dy, x + dx);
                    if y2 >= 0 && y2 < self.grid.len() as i32 && x2 >= 0 && x2 < self.grid[0].len() as i32 {
                        if self.get_char_at(y2, x2) != &'#' {
                            heap.push(State {
                                y: y2,
                                x: x2,
                                direction: StateDirection::Index(dir),
                                cost: cost + 1,
                            });
                        }
                        heap.extend([
                            State {
                                y,
                                x,
                                direction: StateDirection::Index((dir + 1) % 4),
                                cost: cost + 1000,
                            },
                            State {
                                y,
                                x,
                                direction: StateDirection::Index((dir + 3) % 4),
                                cost: cost + 1000,
                            },
                        ]);
                    }
                }
            }
        }
        best
    }

    fn is_within_bounds(&self, y: i32, x: i32) -> bool {
        y >= 0 && y < self.grid.len() as i32 && x >= 0 && x < self.grid[0].len() as i32
    }

    fn get_char_at(&self, y: i32, x: i32) -> &char {
        self.grid
            .get(y as usize)
            .and_then(|c| c.get(x as usize))
            .unwrap_or(&'#')
    }

    fn get_start_and_end(&self) -> ((i32, i32), (i32, i32)) {
        match (self.find_char('S'), self.find_char('E')) {
            (Some(start), Some(end)) => (start, end),
            _ => panic!("Start 'S' or end 'E' position not found in map"),
        }
    }
}

fn get_value(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let maze = Maze::new(&file_contents);

    if part == Part1 {
        maze.find_shortest_path_score() as usize
    } else {
        maze.get_number_of_tiles_on_shortest_paths()
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
        assert_eq!(value, 11048);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 91464);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 64);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 494);
    }
}
