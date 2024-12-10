use std::fs;
use crate::Part::{Part1, Part2};

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

fn read_filesystem(input: &str) -> Vec<u64> {
    let line = input.lines().next().expect("Input should not be empty");
    let mut disk = Vec::new();
    let mut next_index: u64 = 0;
    
    for chunk in line.chars().collect::<Vec<_>>().chunks(2) {
        let count = chunk[0].to_digit(10).expect("Expected digit") as usize;
        
        disk.extend(std::iter::repeat(next_index).take(count));
        next_index += 1;
        
        if chunk.len() > 1 {
            let empty_count = chunk[1].to_digit(10).expect("Expected digit") as usize;
            disk.extend(std::iter::repeat(u64::MAX).take(empty_count));
        }
    }
    
    disk
}

fn defragment(disk: &mut Vec<u64>) {
    for i in (1..disk.len()).rev() {
        let id = disk[i];
        if id == u64::MAX {
            continue;
        }
        
        if let Some(empty_pos) = disk[..i].iter().position(|&x| x == u64::MAX) {
            disk[empty_pos] = id;
            disk[i] = u64::MAX;
        }
    }
}

fn get_checksum(file_path: &str, part: Part) -> u64 {
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut disk = read_filesystem(&file_contents);
    defragment(&mut disk);

    disk.iter()
        .enumerate()
        .filter(|(_, &id)| id != u64::MAX)
        .map(|(i, &id)| id * i as u64)
        .sum()
}

fn main() {
    println!("Value: {}", get_checksum("./test.txt", Part1));
}

#[cfg(test)]
mod tests {
    use crate::get_checksum;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data() {
        let checksum = get_checksum("./test.txt", Part1);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn returns_expected_value_for_input_data() {
        let checksum = get_checksum("./input.txt", Part1);
        assert_eq!(checksum, 6349606724455);
    }
}