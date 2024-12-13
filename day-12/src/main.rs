use crate::Part::{Part1, Part2};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn get_total_price_of_fencing(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut grid: HashMap<(i64, i64), char> = HashMap::new();

    for (row, line) in file_contents.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            grid.insert((row as i64, col as i64), char);
        }
    }

    let mut garden = grid.clone();

    if part == Part1 {
        let mut total = 0;

        while let Some(plot) = garden.keys().copied().next() {
            let (area, perimeter) = find_plot(&mut garden, plot);
            total += area * perimeter;
        }

        total
    } else {
        4
    }
}

fn find_plot(garden: &mut HashMap<(i64, i64), char>, position: (i64, i64)) -> (usize, usize) {
    let mut stack = vec![position];
    let mut visited = HashSet::new();
    let plant = *garden.get(&position).unwrap();

    garden.remove(&position);
    while let Some(location) = stack.pop() {
        if visited.insert(location) {
            for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let new_location = (location.0 + direction.0, location.1 + direction.1);
                if let Some(new_position) = garden.get(&new_location) {
                    if *new_position == plant {
                        garden.remove(&new_location);
                        stack.push(new_location);
                    }
                }
            }
        }
    }

    let mut perimeter = 0;

    for plot in &visited {
        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_location = (plot.0 + direction.0, plot.1 + direction.1);
            if !visited.contains(&new_location) {
                perimeter += 1;
            }
        }
    }

    (visited.len(), perimeter)
}

fn main() {
    println!(
        "Part 1 value: {}",
        get_total_price_of_fencing("./input.txt", Part1)
    );
    println!(
        "Part 2 value: {}",
        get_total_price_of_fencing("./input.txt", Part2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_total_price_of_fencing;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_total_price_of_fencing("./test.txt", Part1);
        assert_eq!(value, 1930);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_total_price_of_fencing("./input.txt", Part1);
        assert_eq!(value, 1431316);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_total_price_of_fencing("./test.txt", Part2);
        assert_eq!(value, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_total_price_of_fencing("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}
