use crate::Part::{Part1, Part2};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn is_collinear(p1: (usize, usize), p2: (usize, usize), p3: (usize, usize)) -> bool {
    let (y1, x1) = p1;
    let (y2, x2) = p2;
    let (y3, x3) = p3;

    let (y1, x1, y2, x2, y3, x3) = (
        y1 as i32, x1 as i32, y2 as i32, x2 as i32, y3 as i32, x3 as i32,
    );

    let (v1x, v1y) = (x2 - x1, y2 - y1);
    let (v2x, v2y) = (x3 - x1, y3 - y1);

    v1x * v2y == v1y * v2x
}

fn get_unique_locations_containing_antinodes(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let map: Vec<Vec<char>> = file_contents.lines().map(|l| l.chars().collect()).collect();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '.' {
                antennas.entry(map[y][x]).or_default().push((y, x))
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    if part == Part1 {
        for antenna in antennas.values() {
            for (y1, x1) in antenna.iter() {
                for (y2, x2) in antenna.iter() {
                    if (y1, x1) == (y2, x2) {
                        continue;
                    }
                    let y = (y2 + y2).wrapping_sub(*y1) as i32;
                    let x = (x2 + x2).wrapping_sub(*x1) as i32;
                    if y >= 0 && y < map.len() as i32 && x >= 0 && x < map[0].len() as i32 {
                        antinodes.insert((y, x));
                    }
                }
            }
        }
    } else {
        for row in 0..map.len() {
            for col in 0..map[row].len() {
                let current_point = (row, col);

                for antenna_points in antennas.values() {
                    for (p1, p2) in antenna_points.iter().tuple_combinations::<(_, _)>() {
                        if is_collinear(*p1, *p2, current_point) {
                            antinodes.insert((row as i32, col as i32));
                            break;
                        }
                    }
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    println!(
        "Part 1 - Unique Antinodes Count: {}",
        get_unique_locations_containing_antinodes("./test.txt", Part1)
    );
    println!(
        "Part 2 - Unique Antinodes Count: {}",
        get_unique_locations_containing_antinodes("./test.txt", Part2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_unique_locations_containing_antinodes;
    use crate::{Part::Part1, Part::Part2};

    #[test]
    fn returns_expected_unique_locations_containing_antinodes_countvalue_test_data_for_part_1() {
        let unique_antinodes_count = get_unique_locations_containing_antinodes("./test.txt", Part1);
        assert_eq!(unique_antinodes_count, 14);
    }

    #[test]
    fn returns_expected_unique_locations_containing_antinodes_count_for_input_data_for_part_1() {
        let unique_antinodes_count =
            get_unique_locations_containing_antinodes("./input.txt", Part1);
        assert_eq!(unique_antinodes_count, 278);
    }

    #[test]
    fn returns_expected_unique_locations_containing_antinodes_countvalue_test_data_for_part_2() {
        let unique_antinodes_count = get_unique_locations_containing_antinodes("./test.txt", Part2);
        assert_eq!(unique_antinodes_count, 34);
    }

    #[test]
    fn returns_expected_unique_locations_containing_antinodes_count_for_input_data_for_part_2() {
        let unique_antinodes_count =
            get_unique_locations_containing_antinodes("./input.txt", Part2);
        assert_eq!(unique_antinodes_count, 1067);
    }
}
