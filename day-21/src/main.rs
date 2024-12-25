use crate::Part::{Part1, Part2};
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Key {
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    Up,
    Right,
    Down,
    Left,
    Empty,
}

use Key::*;

struct Keypad {
    layout: HashMap<(usize, usize), Key>,
}

impl Keypad {
    fn numpad() -> Self {
        let layout = vec![
            ((0, 0), Num7),
            ((0, 1), Num8),
            ((0, 2), Num9),
            ((1, 0), Num4),
            ((1, 1), Num5),
            ((1, 2), Num6),
            ((2, 0), Num1),
            ((2, 1), Num2),
            ((2, 2), Num3),
            ((3, 0), Empty),
            ((3, 1), Num0),
            ((3, 2), A),
        ]
        .into_iter()
        .collect();
        Self { layout }
    }

    fn control() -> Self {
        let layout = vec![
            ((0, 0), Empty),
            ((0, 1), Up),
            ((0, 2), A),
            ((1, 0), Left),
            ((1, 1), Down),
            ((1, 2), Right),
        ]
        .into_iter()
        .collect();
        Self { layout }
    }

    fn find_all_shortest_paths(&self) -> HashMap<Key, HashMap<Key, Vec<Vec<Key>>>> {
        self.layout
            .iter()
            .map(|(coord, key)| (*key, self.find_shortest_paths(*coord)))
            .collect()
    }

    fn find_shortest_paths(&self, start: (usize, usize)) -> HashMap<Key, Vec<Vec<Key>>> {
        let mut paths: HashMap<Key, Vec<Vec<Key>>> = HashMap::new();
        let mut queue: VecDeque<((usize, usize), Vec<Key>)> = vec![(start, vec![])].into();
        let start_key = self.layout[&start];
        paths.insert(start_key, vec![]);

        while let Some((coord, path)) = queue.pop_front() {
            let current_key = self.layout[&coord];
            let shortest = paths.entry(current_key).or_default();

            let mut deduped_shortest = shortest.last().unwrap_or(&vec![]).clone();
            deduped_shortest.dedup();
            let mut deduped_path = path.clone();
            deduped_path.dedup();

            if deduped_shortest.is_empty() || deduped_shortest.len() > deduped_path.len() {
                *shortest = vec![path.clone()];
            } else if deduped_shortest.len() == deduped_path.len() {
                shortest.push(path.clone());
            } else {
                continue;
            }

            for (neighbor, direction) in self.get_neighbors(coord) {
                if let Some(&key) = self.layout.get(&neighbor) {
                    if key != Empty && key != start_key {
                        let mut next_path = path.clone();
                        next_path.push(direction);
                        queue.push_back((neighbor, next_path));
                    }
                }
            }
        }

        for paths in paths.values_mut() {
            for path in paths.iter_mut() {
                path.push(A);
            }
        }
        paths
    }

    fn get_neighbors(&self, coord: (usize, usize)) -> [((usize, usize), Key); 4] {
        [
            ((coord.0.wrapping_sub(1), coord.1), Up),
            ((coord.0, coord.1 + 1), Right),
            ((coord.0 + 1, coord.1), Down),
            ((coord.0, coord.1.wrapping_sub(1)), Left),
        ]
    }
}

struct PathFinder {
    numpad_paths: HashMap<Key, HashMap<Key, Vec<Vec<Key>>>>,
    control_paths: HashMap<Key, HashMap<Key, Vec<Vec<Key>>>>,
}

impl PathFinder {
    fn new() -> Self {
        let numpad_paths = Keypad::numpad().find_all_shortest_paths();
        let control_paths = Keypad::control().find_all_shortest_paths();
        Self {
            numpad_paths,
            control_paths,
        }
    }

    fn find_complexity(&self, code: Vec<Key>, value: usize, max_depth: usize) -> usize {
        let mut previous_key = A;
        let mut total_len = 0;

        for key in code {
            total_len += self.find_recursive(
                (previous_key, key),
                max_depth,
                0,
                &mut HashMap::new(),
                &mut HashMap::new(),
            );
            previous_key = key;
        }

        value * total_len
    }

    fn find_recursive(
        &self,
        path: (Key, Key),
        max: usize,
        current: usize,
        memo: &mut HashMap<(Key, Key), HashMap<usize, usize>>,
        last_at_level: &mut HashMap<usize, Key>,
    ) -> usize {
        if current == max {
            return 1;
        }

        if let Some(cached) = memo.get(&path).and_then(|m| m.get(&current)) {
            return *cached;
        }

        let paths = if current == 0 {
            &self.numpad_paths
        } else {
            &self.control_paths
        }
        .get(&path.0)
        .unwrap()
        .get(&path.1)
        .unwrap();

        let last = *last_at_level.entry(current).or_insert(A);
        let mut next_last = last;
        let mut min_total = usize::MAX;

        for possible_path in paths {
            let mut sub_total = 0;
            let mut previous = last;

            for &part in possible_path {
                sub_total +=
                    self.find_recursive((previous, part), max, current + 1, memo, last_at_level);
                previous = part;
            }

            if sub_total < min_total {
                min_total = sub_total;
                next_last = *possible_path.last().unwrap();
            }
        }

        last_at_level.insert(current, next_last);
        memo.entry(path).or_default().insert(current, min_total);
        min_total
    }
}

fn parse_input(file_path: &str) -> Vec<(Vec<Key>, usize)> {
    fs::read_to_string(file_path)
        .expect("Should have been able to read the file")
        .trim()
        .lines()
        .map(|line| {
            (
                line.chars()
                    .map(|c| match c {
                        '0' => Num0,
                        '1' => Num1,
                        '2' => Num2,
                        '3' => Num3,
                        '4' => Num4,
                        '5' => Num5,
                        '6' => Num6,
                        '7' => Num7,
                        '8' => Num8,
                        '9' => Num9,
                        'A' => A,
                        _ => unreachable!(),
                    })
                    .collect(),
                line[0..3].parse().unwrap(),
            )
        })
        .collect()
}

fn get_value(file_path: &str, part: Part) -> usize {
    let codes = parse_input(file_path);
    let path_finder = PathFinder::new();

    let max_depth = if part == Part1 { 3 } else { 26 };

    codes
        .into_iter()
        .map(|(code, value)| path_finder.find_complexity(code, value, max_depth))
        .sum()
}

fn main() {
    println!("Part 1 value: {}", get_value("./input.txt", Part1));
    println!("Part 2 value: {}", get_value("./input.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::get_value;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let value = get_value("./test.txt", Part1);
        assert_eq!(value, 126384);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 248684);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 154115708116294);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 307055584161760);
    }
}
