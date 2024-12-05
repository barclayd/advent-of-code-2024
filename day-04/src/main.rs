use std::fs;
use std::collections::HashSet;

fn get_position_of_word_in_grid(grid: &Vec<Vec<char>>, search_word: &str) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();
    let first_char = search_word.chars().next().unwrap();
    let word_chars: Vec<char> = search_word.chars().collect();

    let mut results = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == first_char {
                let found_positions = breadth_first_search(&grid, row, col, &word_chars);
                results.extend(found_positions);
            }
        }
    }

    results
}

fn breadth_first_search(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, word_chars: &[char]) -> Vec<(usize, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = HashSet::new();
    let mut results = Vec::new();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (dx, dy) in directions {
        let mut current_visited = HashSet::new();
        let mut current_path = vec![(start_row, start_col)];
        current_visited.insert((start_row, start_col));
        
        let mut row = start_row;
        let mut col = start_col;
        let mut found = true;
        
        for i in 1..word_chars.len() {
            let new_row = match (row as isize).checked_add(dx) {
                Some(r) if r >= 0 && (r as usize) < rows => r as usize,
                _ => { found = false; break }
            };

            let new_col = match (col as isize).checked_add(dy) {
                Some(c) if c >= 0 && (c as usize) < cols => c as usize,
                _ => { found = false; break }
            };

            if grid[new_row][new_col] != word_chars[i] {
                found = false;
                break;
            }

            current_path.push((new_row, new_col));
            current_visited.insert((new_row, new_col));
            row = new_row;
            col = new_col;
        }

        if found {
            results.push(current_path[0]);
            visited.extend(current_visited);
        }
    }
    
    results
}

fn get_word_search_count(file_path: &str, search_word: &str) -> i32 {
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let positions = get_position_of_word_in_grid(&grid, search_word);
    positions.len() as i32
}

fn get_word_search_from_x_formation_count(file_path: &str, search_word: &str) -> i32 {
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut x_formation_count = 0;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if get_is_x_formation(&grid, x as i32, y as i32, search_word) {
                x_formation_count += 1
            }
        }
    }

    x_formation_count
}

fn get_is_x_formation(chars: &Vec<Vec<char>>, x: i32, y: i32, search_word: &str) -> bool {
    let c = get_char_at(chars, x, y);
    let tl = get_char_at(chars, x - 1, y - 1);
    let tr = get_char_at(chars, x + 1, y - 1);
    let bl = get_char_at(chars, x - 1, y + 1);
    let br = get_char_at(chars, x + 1, y + 1);

    [format!("{tl}{c}{br}"), format!("{bl}{c}{tr}")]
        .iter()
        .all(|word| word == search_word || *word == search_word.chars().rev().collect::<String>())
}

fn get_char_at(chars: &Vec<Vec<char>>, x: i32, y: i32) -> char {
    *chars
        .get(x as usize)
        .and_then(|c| c.get(y as usize))
        .unwrap_or(&'.')
}


fn main() {
    let count = get_word_search_count("./input.txt", "XMAS");
    println!("Part 1: {}", count);
    let count_from_x_formation = get_word_search_from_x_formation_count("./input.txt", "XMAS");
    println!("Part 2: {}", count_from_x_formation);
}

#[cfg(test)]
mod tests {
    use crate::{get_word_search_count, get_word_search_from_x_formation_count};

    #[test]
    fn returns_expected_word_search_count_for_test_data() {
        let count = get_word_search_count("./test.txt", "XMAS");
        assert_eq!(count, 18);
    }

    #[test]
    fn returns_expected_word_search_count_for_input_data() {
        let count = get_word_search_count("./input.txt", "XMAS");
        assert_eq!(count, 2500);
    }

    #[test]
    fn returns_expected_word_search_count_in_x_formation_for_test_data() {
        let count = get_word_search_from_x_formation_count("./test.txt", "MAS");
        assert_eq!(count, 9);
    }

    #[test]
    fn returns_expected_word_search_count_in_x_formation_for_input_data() {
        let count = get_word_search_from_x_formation_count("./input.txt", "MAS");
        assert_eq!(count, 1933);
    }
}