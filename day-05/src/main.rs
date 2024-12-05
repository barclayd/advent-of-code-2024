use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs;
use crate::Part::{Part1, Part2};

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn build_dependency_graph(rules: &[(i32, i32)]) -> HashMap<i32, HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for &(before, after) in rules {
        graph.entry(before).or_default();
        graph.entry(after).or_default().insert(before);
    }

    graph
}

fn parse_page_number_rules(input: &str) -> Result<(i32, i32), String> {
    let numbers: Result<Vec<i32>, _> = input.split('|').map(str::parse::<i32>).collect();

    match numbers {
        Ok(nums) if nums.len() == 2 => Ok((nums[0], nums[1])),
        _ => Err("Invalid input format".to_string()),
    }
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

fn reorder_sequence(sequence: &[i32], graph: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut reordered_sequence: Vec<i32> = sequence.to_vec();

    for &page in sequence {
        if let Some(prerequisites) = graph.get(&page) {
            let mut relevant_prerequisites: Vec<_> = prerequisites
            .iter()
            .filter(|prerequisite| sequence.contains(prerequisite))
                .copied()
                .collect();

                relevant_prerequisites.sort();

            for &prerequisite in &relevant_prerequisites {
                let current_pos = reordered_sequence.iter().position(|&p| p == prerequisite);
                let page_pos = reordered_sequence.iter().position(|&p| p == page);
                
                if let (Some(curr_pos), Some(p_pos)) = (current_pos, page_pos) {
                    if curr_pos > p_pos {
                        reordered_sequence.remove(curr_pos);
                        reordered_sequence.insert(p_pos, prerequisite);
                    }
                }
            }
        }
    }
    
    reordered_sequence
}

fn get_sum_of_middle_pages_for_valid_sequences(file_path: &str, part: Part) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (rules, updates) = file_contents.split_once("\n\n").unwrap();

    let parsed_rules = rules
        .lines()
        .flat_map(parse_page_number_rules)
        .collect::<Vec<(i32, i32)>>();

    let parsed_updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let graph = build_dependency_graph(&parsed_rules);

    let part_1: i32 = parsed_updates
        .iter()
        .filter_map(|sequence| {
            is_valid_update_sequence(sequence, &graph).then(|| get_middle_page(sequence))
        })
        .sum();

    let part_2: i32 = parsed_updates
        .iter()
        .filter_map(|sequence| {
            let is_valid = is_valid_update_sequence(sequence, &graph);
            if is_valid {
                return None;
            }
            let reordered_sequence = reorder_sequence(sequence, &graph);
            Some(get_middle_page(reordered_sequence.as_slice()))
        })
        .sum();
    
    if part == Part::Part1 {
        return part_1;
    }
    
    part_2
}

fn main() {
    println!(
        "Part 1: Sum of middle pages: {}",
        get_sum_of_middle_pages_for_valid_sequences("./input.txt", Part1)
    );
    println!(
        "Part 2: Sum of middle pages: {}",
        get_sum_of_middle_pages_for_valid_sequences("./input.txt", Part2)
    );
}

#[cfg(test)]
mod tests {
    use crate::get_sum_of_middle_pages_for_valid_sequences;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let sum_of_middle_pages = get_sum_of_middle_pages_for_valid_sequences("./test.txt", Part1);
        assert_eq!(sum_of_middle_pages, 143);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let sum_of_middle_pages = get_sum_of_middle_pages_for_valid_sequences("./input.txt", Part1);
        assert_eq!(sum_of_middle_pages, 6051);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let sum_of_middle_pages = get_sum_of_middle_pages_for_valid_sequences("./test.txt", Part2);
        assert_eq!(sum_of_middle_pages, 123);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let sum_of_middle_pages = get_sum_of_middle_pages_for_valid_sequences("./input.txt", Part2);
        assert_eq!(sum_of_middle_pages, 5390);
    }
}
