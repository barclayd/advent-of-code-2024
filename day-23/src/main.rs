use std::collections::HashMap;
use std::fs;

use itertools::Itertools;
use rand::seq::SliceRandom;

fn get_graph(file_path: &str) -> HashMap<String, Vec<String>> {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    file_contents
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .for_each(|(a, b)| {
            graph.entry(a.to_string()).or_default().push(b.to_string());
            graph.entry(b.to_string()).or_default().push(a.to_string());
        });

    graph
}

fn get_password(file_path: &str) -> String {
    let graph = get_graph(file_path);

    let highest_degree = graph.values().map(|v| v.len()).max().unwrap();
    let mut max_clique = Vec::new();
    while max_clique.len() < highest_degree {
        let mut ks = graph.keys().collect_vec();
        ks.shuffle(&mut rand::rng());

        let mut clique: Vec<&str> = Vec::new();
        for k in ks {
            if clique.iter().all(|c| graph[*c].contains(k)) {
                let _ = &clique.push(k);
            }
        }

        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
    }

    max_clique.iter().sorted().join(",")
}

fn get_tri_clique(file_path: &str) -> i32 {
    let graph = get_graph(file_path);

    let ks = graph.keys().sorted().collect_vec();
    let mut tri_cliques = 0;
    for (i, a) in ks.iter().enumerate() {
        for j in (i + 1)..ks.len() {
            for k in (j + 1)..ks.len() {
                let b = ks[j];
                let c = ks[k];
                if (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                    && graph[b].contains(a)
                    && graph[c].contains(a)
                    && graph[c].contains(b)
                {
                    tri_cliques += 1;
                }
            }
        }
    }

    tri_cliques
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
