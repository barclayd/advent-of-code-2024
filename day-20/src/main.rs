use crate::Part::{Part1, Part2};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    #[default]
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn all() -> &'static [Direction; 4] {
        &[Direction::UP, Direction::RIGHT, Direction::DOWN, Direction::LEFT]
    }

    pub fn unit(&self) -> (i64, i64) {
        match self {
            Direction::UP => (-1, 0),
            Direction::RIGHT => (0, 1),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
        }
    }
}

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Default)]
pub struct Race {
    size: (i64, i64),
    start: (i64, i64),
    end: (i64, i64),
    maze: HashSet<(i64, i64)>,
    path: VecDeque<(i64, i64)>,
}

impl Race {
    fn find_path(&mut self) -> Option<()> {
        let mut queue = VecDeque::from([(0, self.start)]);
        let mut visited = HashSet::new();
        let mut dist = HashMap::<(i64, i64), usize>::new();
        let mut backlink = HashMap::<(i64, i64), (i64, i64)>::new();

        while let Some((cost, position)) = queue.pop_front() {
            if position == self.end {
                break;
            }
            if visited.insert(position) {
                for dir in Direction::all() {
                    let (dy, dx) = dir.unit();
                    let new_position = (position.0 + dy, position.1 + dx);
                    let new_cost = cost + 1;
                    
                    if !self.maze.contains(&new_position) {
                        continue;
                    }

                    if !visited.contains(&new_position) {
                        let entry = dist.entry(new_position).or_insert(usize::MAX);
                        if new_cost < *entry {
                            *entry = new_cost;
                            backlink.insert(new_position, position);
                            queue.push_back((new_cost, new_position));
                        }
                    }
                }
            }
        }

        self.reconstruct_path(&backlink)
    }

    fn reconstruct_path(&mut self, backlink: &HashMap<(i64, i64), (i64, i64)>) -> Option<()> {
        self.path.clear();
        self.path.push_back(self.end);
        let mut current = self.end;
        
        while let Some(&next) = backlink.get(&current) {
            self.path.push_front(next);
            current = next;
        }
        
        Some(())
    }

    fn from_input(lines: &[&str]) -> Self {
        let mut race = Race::default();
        race.size = (lines.len() as i64, lines[0].len() as i64);

        for (row, line) in lines.iter().enumerate() {
            for (col, character) in line.chars().enumerate() {
                let position = (row as i64, col as i64);
                match character {
                    'S' => {
                        race.start = position;
                        race.maze.insert(position);
                    },
                    'E' => {
                        race.end = position;
                        race.maze.insert(position);
                    },
                    '#' => continue,
                    _ => { race.maze.insert(position); }
                }
            }
        }
        race
    }
}

fn get_count_of_ways_to_cheat(race: &Race, picosec: usize) -> usize {
    let mut total = 0;
    for i in 0..race.path.len().saturating_sub(3) {
        for j in i + 3..race.path.len() {
            let manhattan_distance = (race.path[i].0.abs_diff(race.path[j].0)
                + race.path[i].1.abs_diff(race.path[j].1)) as usize;
                
            if manhattan_distance <= picosec && (j - i) > manhattan_distance {
                total += ((j - i) - manhattan_distance >= 100) as usize;
            }
        }
    }
    total
}

fn get_value(file_path: &str, part: Part) -> usize {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    
    let mut race = Race::from_input(&lines);
    race.find_path();

    get_count_of_ways_to_cheat(&race, if part == Part1 { 2 } else { 20 })
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
        assert_eq!(value, 0);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let value = get_value("./input.txt", Part1);
        assert_eq!(value, 1384);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let value = get_value("./test.txt", Part2);
        assert_eq!(value, 0);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 1008542);
    }
}
