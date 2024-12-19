use pathfinding::prelude::astar;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Grid {
    width: i64,
    height: i64,
    obstacles: Vec<(i64, i64)>,
}

impl Grid {
    const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    const START: (i64, i64) = (0, 0);
    const END: (i64, i64) = (70, 70);

    fn new(input: &str) -> Self {
        let obstacles = input
            .lines()
            .filter_map(|line| {
                line.split_once(',')
                    .and_then(|(x, y)| Some((
                        x.trim().parse().ok()?,
                        y.trim().parse().ok()?
                    )))
            })
            .collect();

        Self {
            width: 70,
            height: 70,
            obstacles,
        }
    }

    fn search(&self, limit: usize) -> Option<(Vec<(i64, i64)>, i64)> {
        let memory: HashSet<&(i64, i64)> = self.obstacles.iter().take(limit).collect();

        astar(
            &Self::START,
            |&state| {
                Self::DIRECTIONS.iter()
                    .map(|&dir| ((state.0 + dir.0, state.1 + dir.1), 1))
                    .filter(|(pos, _)| !memory.contains(pos))
                    .filter(|(pos, _)| self.is_within_bounds(*pos))
                    .collect::<Vec<_>>()
            },
            |state| (Self::END.0 - state.0) + (Self::END.1 - state.1),
            |state| *state == Self::END,
        )
    }

    fn is_within_bounds(&self, pos: (i64, i64)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && pos.0 <= self.width && pos.1 <= self.height
    }

    fn find_critical_coordinate(&self) -> (i64, i64) {
        let mut min = 0;
        let mut max = self.obstacles.len();

        while max > min {
            let mid = (max + min) / 2;
            if self.search(mid).is_none() {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        self.obstacles[max - 1]
    }
}

fn get_minimum_steps(file_path: &str) -> i64 {
    let grid = Grid::new(&fs::read_to_string(file_path).expect("Failed to read file"));

    match grid.search(1024) {
        Some((_, cost)) => cost,
        None => panic!("No solution found"),
    }
}

fn get_coordinates(file_path: &str) -> String {
    let grid = Grid::new(&fs::read_to_string(file_path).expect("Failed to read file"));
    let (x, y) = grid.find_critical_coordinate();
    format!("{},{}", x, y)
}
fn main() {
    println!("Part 1 value: {}", get_minimum_steps("./input.txt"));
    println!("Part 2 value: {}", get_coordinates("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::{get_coordinates, get_minimum_steps};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_minimum_steps("./test.txt");
        assert_eq!(value, 146);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_minimum_steps("./input.txt");
        assert_eq!(value, 374);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_coordinates("./input.txt");
        assert_eq!(value, "30,12");
    }
}
