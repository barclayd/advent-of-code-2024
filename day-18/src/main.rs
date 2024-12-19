use pathfinding::prelude::astar;
use std::collections::HashSet;
use std::fs;

const DIRS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn get_minimum_steps(file_path: &str) -> i64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let list: Vec<(i64, i64)> = file_contents
        .lines()
        .filter_map(|line| {
            line.split_once(',')
                .and_then(|(x, y)| Some((x.trim().parse().ok()?, y.trim().parse().ok()?)))
        })
        .collect();

    let memory: HashSet<&(i64, i64)> = list.iter().take(1024).collect();
    let Some((_path, cost)) = astar(
        &(0i64, 0i64),
        |state| {
            DIRS.iter()
                .map(|dir| ((state.0 + dir.0, state.1 + dir.1), 1))
                .filter(|(pos, _)| !memory.contains(&pos))
                .filter(|(pos, _)| pos.0 >= 0 && pos.1 >= 0 && pos.0 <= 70 && pos.1 <= 70)
                .collect::<Vec<_>>()
        },
        |state| (70 - state.0) + (70 - state.1),
        |state| *state == (70, 70),
    ) else {
        panic!("No path found")
    };

    cost
}

fn main() {
    println!("Part 1 value: {}", get_minimum_steps("./input.txt"));
    println!("Part 2 value: {}", get_minimum_steps("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::get_minimum_steps;

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
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_minimum_steps("./test.txt");
        assert_eq!(value, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_minimum_steps("./input.txt");
        assert_eq!(value, 4);
    }
}
