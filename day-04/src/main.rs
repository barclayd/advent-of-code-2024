use std::fs;

const CHARACTER_DIRECTIONS: [(i32, i32); 8] = [(-1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1), (1, 0), (1, 1), (1, -1)];

fn get_word_count(grid: &Vec<Vec<char>>, x: usize, y: usize, search_word: &str) -> usize {
    let mut count: usize = 0;

    for (dx, dy) in CHARACTER_DIRECTIONS.iter() {
        let mut word: Vec<char> = vec!['.'; search_word.len()];
        for i in 0..search_word.len() {
            word[i] = *grid
                .get(x.wrapping_add((dx * i as i32) as usize))
                .and_then(|c| c.get(y.wrapping_add((dy * i as i32) as usize)))
                .unwrap_or(&'.');
        }
        if word.iter().collect::<String>() == search_word {
            count += 1
        }
    }

    count
}

fn get_word_search_count(file_path: &str, search_word: &str) -> usize {
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            count += get_word_count(&grid, row, col, search_word)
        }
    }

    count
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