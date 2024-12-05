use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_page_number_rules(input: &str) -> Result<(i32, i32), String> {
    let numbers: Result<Vec<i32>, _> = input.split('|').map(str::parse::<i32>).collect();

    match numbers {
        Ok(nums) if nums.len() == 2 => Ok((nums[0], nums[1])),
        _ => Err("Invalid input format".to_string()),
    }
}

fn build_dependency_graph(rules: &[(i32, i32)]) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for &(before, after) in rules {
        graph.entry(before).or_default();
        graph.entry(after).or_default().insert(before);
    }

    graph
}

fn get_middle_page(sequence: &[i32]) -> i32 {
    let middle_index = (sequence.len() - 1) / 2;
    sequence[middle_index]
}

fn is_valid_update_sequence(sequence: &[i32], graph: &HashMap<i32, HashSet<i32>>) -> bool {
    for (index, &page) in sequence.iter().enumerate() {
        let pages_seen: HashSet<i32> = sequence[..index].iter().copied().collect();

        if let Some(prerequisites) = graph.get(&page) {
            let relevant_prerequisites: HashSet<_> = prerequisites
                .iter()
                .filter(|prerequisite| sequence.contains(prerequisite))
                .copied()
                .collect();

            if !relevant_prerequisites.is_subset(&pages_seen) {
                return false;
            }
        }
    }
    true
}

fn get_sum_of_middle_pages_for_valid_sequences(file_path: &str) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let page_number_rules = file_contents
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .flat_map(parse_page_number_rules)
        .collect::<Vec<(i32, i32)>>();

    let update_sequence = file_contents
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let graph = build_dependency_graph(&page_number_rules);

    update_sequence
        .iter()
        .filter_map(|sequence| {
            is_valid_update_sequence(sequence, &graph).then(|| get_middle_page(sequence))
        })
        .sum()
}

fn main() {
    println!(
        "Sum of middle pages: {}",
        get_sum_of_middle_pages_for_valid_sequences("./input.txt")
    );
}

#[cfg(test)]
mod tests {
    use crate::get_sum_of_middle_pages_for_valid_sequences;

    #[test]
    fn returns_expected_value_test_data() {
        let sum_of_middle_pages = get_sum_of_middle_pages_for_valid_sequences("./test.txt");
        assert_eq!(sum_of_middle_pages, 143);
    }

    #[test]
    fn returns_expected_value_for_input_data() {
        let sum_of_middle_pages = get_sum_of_middle_pages_for_valid_sequences("./input.txt");
        assert_eq!(sum_of_middle_pages, 6051);
    }
}
