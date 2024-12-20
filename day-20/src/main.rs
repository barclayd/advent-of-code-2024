use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use crate::Part::{Part1, Part2};

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    #[default]
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub const DIRS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
];

impl Direction {
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
    path: HashSet<(i64, i64)>
}

fn get_value(file_path: &str, part: Part) -> usize {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let lines: Vec<&str> = file_contents.lines().collect();
    
    let mut race = Race::default();
    race.size = (lines.len() as i64, lines[0].len() as i64);
    
    for (row, line) in file_contents.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            let position = (row as i64, col as i64);
            if character == 'S' {
                race.start = position;
            } else if character == 'E' {
                race.end = position;
            }
            if character != '#' {
                race.path.insert(position);
            }
        }
    }

   if part == Part1 {
       let mut queue = VecDeque::from([(0, race.start)]);
       let mut visited = HashSet::new();

       let mut dist = HashMap::<(i64, i64), usize>::new();
       let mut backlink = HashMap::<(i64, i64), (i64, i64)>::new();

       while let Some((cost, position)) = queue.pop_front() {
           if position == race.end {
               break;
           }
           if visited.insert(position) {
               for dir in &DIRS {
                   let new_position = (position.0 + dir.unit().0, position.1 + dir.unit().1);
                   let cost = cost + 1;
                   if !race.path.contains(&new_position) {
                       continue;
                   }

                   if !visited.contains(&new_position) {
                       let entry = dist.entry(position).or_insert(usize::MAX);
                       if cost < *entry {
                           *entry = cost;
                           backlink.insert(new_position, position);
                       }
                   }
                   queue.push_back((cost, new_position));
               }
           }
       }

       let mut path = VecDeque::from([race.end]);
       let mut cur = race.end;
       while let Some(next) = backlink.get(&cur) {
           path.push_front(*next);
           cur = *next;
       }

       let mut cheats = HashMap::<usize, usize>::new();
       for i in 0..path.len() - 1 {
           for j in i + 1..path.len() {
               let dist: usize = (path[i].0.abs_diff(path[j].0) + path[i].1.abs_diff(path[j].1)) as usize;
               if dist < 3 && (j - i) > dist {
                   *cheats.entry((j - i) - dist).or_default() += 1;
               }
           }
       }

       cheats
           .iter()
           .filter_map(|(picosec, ways)| if *picosec >= 100 { Some(ways) } else { None })
           .sum::<usize>()
   } else { 4 }
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
        assert_eq!(value, 4);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let value = get_value("./input.txt", Part2);
        assert_eq!(value, 4);
    }
}