use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use rand::seq::SliceRandom;

struct Graph {
    adjacency: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from_file(file_path: &str) -> Self {
        let file_contents = 
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        file_contents
            .lines()
            .filter_map(|l| l.split_once('-'))
            .for_each(|(a, b)| {
                adjacency.entry(a.to_string()).or_default().push(b.to_string());
                adjacency.entry(b.to_string()).or_default().push(a.to_string());
            });

        Self { adjacency }
    }

    fn find_max_clique(&self) -> Vec<String> {
        let highest_degree = self.adjacency.values().map(|v| v.len()).max().unwrap();
        let mut max_clique = Vec::new();

        while max_clique.len() < highest_degree {
            let mut candidates = self.adjacency.keys().cloned().collect_vec();
            candidates.shuffle(&mut rand::rng());

            let mut current_clique = Vec::new();
            for candidate in candidates {
                if current_clique.iter()
                    .all(|c| self.adjacency[c].contains(&candidate)) 
                {
                    current_clique.push(candidate);
                }
            }

            if current_clique.len() > max_clique.len() {
                max_clique = current_clique;
            }
        }

        max_clique
    }

    fn count_tri_cliques(&self) -> i32 {
        let vertices = self.adjacency.keys().sorted().collect_vec();
        
        let mut count = 0;
        for (i, a) in vertices.iter().enumerate() {
            for (j, b) in vertices[i + 1..].iter().enumerate() {
                for c in vertices[i + j + 2..].iter() {
                    if self.is_tri_clique(a, b, c) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn is_tri_clique(&self, a: &&String, b: &&String, c: &&String) -> bool {
        (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
            && self.adjacency[*b].contains(*a)
            && self.adjacency[*c].contains(*a)
            && self.adjacency[*c].contains(*b)
    }
}

fn get_password(file_path: &str) -> String {
    let graph = Graph::from_file(file_path);
    graph.find_max_clique()
        .iter()
        .sorted()
        .join(",")
}

fn get_tri_clique(file_path: &str) -> i32 {
    let graph = Graph::from_file(file_path);
    graph.count_tri_cliques()
}

fn main() {
    println!("Part 1 value: {}", get_tri_clique("./input.txt"));
    println!("Part 2 value: {}", get_password("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::{get_tri_clique, get_password};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_tri_clique("./test.txt");
        assert_eq!(value, 7);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_tri_clique("./input.txt");
        assert_eq!(value, 1306);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_password("./test.txt");
        assert_eq!(value, "co,de,ka,ta");
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_password("./input.txt");
        assert_eq!(value, "bd,dk,ir,ko,lk,nn,ob,pt,te,tl,uh,wj,yl");
    }
}
