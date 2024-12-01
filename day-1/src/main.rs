use std::fs;

fn get_total_distance_between_lists(file_path: &str) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lists: (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    for line in file_contents.lines() {
        let mut location_ids = line.split_whitespace();
        if let (Some(first), Some(second)) = (location_ids.next(), location_ids.next()) {
            lists.0.push(first.parse().expect("Should be a number"));
            lists.1.push(second.parse().expect("Should be a number"));
        }
    }

    lists.0.sort_unstable();
    lists.1.sort_unstable();

    lists.0.iter()
        .zip(lists.1.iter())
        .map(|(a, b)| (b - a).abs())
        .sum()
}

fn main() {
    println!("Total distance between lists: {}", get_total_distance_between_lists("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::get_total_distance_between_lists;

    #[test]
    fn returns_expected_total_distance_between_lists_for_test_data() {
        let total_distance_between_lists = get_total_distance_between_lists("./test.txt");
        assert_eq!(total_distance_between_lists, 11);
    }

    #[test]
    fn returns_expected_total_distance_between_lists_for_input_data() {
        let total_distance_between_lists = get_total_distance_between_lists("./input.txt");
        assert_eq!(total_distance_between_lists, 1110981);
    }
}