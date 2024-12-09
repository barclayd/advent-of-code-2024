use std::collections::{HashMap, HashSet};
use std::fs;

fn get_unique_locations_containing_antinodes(file_path: &str) -> usize {
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
    
   antinodes.len()
}

fn main() {
    println!("Unique Antinodes Count: {}", get_unique_locations_containing_antinodes("./test.txt"));
}

#[cfg(test)]
mod tests {
    use crate::get_unique_locations_containing_antinodes;

    #[test]
    fn returns_expected_value_test_data() {
        let unique_antinodes_count = get_unique_locations_containing_antinodes("./test.txt");
        assert_eq!(unique_antinodes_count, 14);
    }

    #[test]
    fn returns_expected_value_for_input_data() {
        let unique_antinodes_count = get_unique_locations_containing_antinodes("./input.txt");
        assert_eq!(unique_antinodes_count, 278);
    }
}