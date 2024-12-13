use crate::Part::{Part1, Part2};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_total_price_of_fencing(file_path: &str, part: Part) -> usize {
    let garden = load_garden_from_file(file_path);
    calculate_total_price(garden, part)
}

fn load_garden_from_file(file_path: &str) -> HashMap<(i64, i64), char> {
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    file_contents
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| ((row as i64, col as i64), char))
        })
        .collect()
}

fn calculate_total_price(mut garden: HashMap<(i64, i64), char>, part: Part) -> usize {
    let mut total = 0;
    
    while let Some(plot) = garden.keys().copied().next() {
        let (area, perimeter) = match part {
            Part1 => find_plot(&mut garden, plot),
            Part2 => find_plot_with_reduced_fencing(&mut garden, plot),
        };
        total += area * perimeter;
    }
    
    total
}

fn find_plot(garden: &mut HashMap<(i64, i64), char>, position: (i64, i64)) -> (usize, usize) {
    let (visited, _) = find_connected_plots(garden, position);
    let perimeter = calculate_perimeter(&visited);
    (visited.len(), perimeter)
}

fn find_connected_plots(
    garden: &mut HashMap<(i64, i64), char>,
    position: (i64, i64),
) -> (HashSet<(i64, i64)>, char) {
    let plant = *garden.get(&position).unwrap();
    let visited = explore_connected_positions(garden, position, plant);
    (visited, plant)
}

fn explore_connected_positions(
    garden: &mut HashMap<(i64, i64), char>,
    start: (i64, i64),
    target: char,
) -> HashSet<(i64, i64)> {
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    
    garden.remove(&start);
    
    while let Some(location) = stack.pop() {
        if visited.insert(location) {
            let neighbors = get_matching_neighbors(garden, location, target);
            for neighbor in neighbors {
                garden.remove(&neighbor);
                stack.push(neighbor);
            }
        }
    }
    
    visited
}

fn get_matching_neighbors(
    garden: &HashMap<(i64, i64), char>,
    location: (i64, i64),
    target: char,
) -> Vec<(i64, i64)> {
    DIRECTIONS.iter()
        .map(|&(dx, dy)| (location.0 + dx, location.1 + dy))
        .filter(|&pos| garden.get(&pos).map_or(false, |&c| c == target))
        .collect()
}

fn calculate_perimeter(visited: &HashSet<(i64, i64)>) -> usize {
    let mut perimeter = 0;
    
    for &plot in visited {
        for direction in DIRECTIONS.iter() {
            let new_location = (plot.0 + direction.0, plot.1 + direction.1);
            if !visited.contains(&new_location) {
                perimeter += 1;
            }
        }
    }
    
    perimeter
}

fn find_plot_with_reduced_fencing(
    garden: &mut HashMap<(i64, i64), char>,
    position: (i64, i64),
) -> (usize, usize) {
    let (visited, _) = find_connected_plots(garden, position);
    
    let mut edge_list = build_edge_list(&visited);

    let perimeter = calculate_reduced_perimeter(&mut edge_list);

    (visited.len(), perimeter)
}

fn build_edge_list(visited: &HashSet<(i64, i64)>) -> HashSet<((i64, i64), (i64, i64))> {
    let mut edge_list = HashSet::new();
    
    for &plot in visited {
        for direction in DIRECTIONS.iter() {
            let new_location = (plot.0 + direction.0, plot.1 + direction.1);
            if !visited.contains(&new_location) {
                edge_list.insert((plot, new_location));
            }
        }
    }

    edge_list
}

fn calculate_reduced_perimeter(edge_list: &mut HashSet<((i64, i64), (i64, i64))>) -> usize {
    let mut perimeter = 0;
    
    while let Some(initial_edge) = edge_list.iter().copied().next() {
        let mut search = initial_edge;
        
        if search.0.0 == search.1.0 {
            search = find_horizontal_line_start(edge_list, search);
        } else {
            search = find_vertical_line_start(edge_list, search);
        }
        
        if search.0.0 == search.1.0 {
            remove_horizontal_line(edge_list, search);
        } else {
            remove_vertical_line(edge_list, search);
        }
        
        perimeter += 1;
    }
    
    perimeter
}

fn find_horizontal_line_start(
    edge_list: &HashSet<((i64, i64), (i64, i64))>,
    mut edge: ((i64, i64), (i64, i64)),
) -> ((i64, i64), (i64, i64)) {
    let mut new_edge = ((edge.0.0 - 1, edge.0.1), (edge.1.0 - 1, edge.1.1));
    while edge_list.contains(&new_edge) {
        edge = new_edge;
        new_edge = ((edge.0.0 - 1, edge.0.1), (edge.1.0 - 1, edge.1.1));
    }
    edge
}

fn find_vertical_line_start(
    edge_list: &HashSet<((i64, i64), (i64, i64))>,
    mut edge: ((i64, i64), (i64, i64)),
) -> ((i64, i64), (i64, i64)) {
    let mut new_edge = ((edge.0.0, edge.0.1 - 1), (edge.1.0, edge.1.1 - 1));
    while edge_list.contains(&new_edge) {
        edge = new_edge;
        new_edge = ((edge.0.0, edge.0.1 - 1), (edge.1.0, edge.1.1 - 1));
    }
    edge
}

fn remove_horizontal_line(
    edge_list: &mut HashSet<((i64, i64), (i64, i64))>,
    mut edge: ((i64, i64), (i64, i64)),
) {
    while edge_list.remove(&edge) {
        edge = ((edge.0.0 + 1, edge.0.1), (edge.1.0 + 1, edge.1.1));
    }
}

fn remove_vertical_line(
    edge_list: &mut HashSet<((i64, i64), (i64, i64))>,
    mut edge: ((i64, i64), (i64, i64)),
) {
    while edge_list.remove(&edge) {
        edge = ((edge.0.0, edge.0.1 + 1), (edge.1.0, edge.1.1 + 1));
    }
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
        assert_eq!(value, 1206);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_total_price_of_fencing("./input.txt", Part2);
        assert_eq!(value, 821428);
    }
}
