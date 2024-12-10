use crate::Part::{Part1, Part2};
use std::fs;

#[derive(PartialEq, Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy)]
struct DiskEntry {
    id: u64,
    size: usize,
}

impl DiskEntry {
    fn new_file(id: u64, size: usize) -> Self {
        Self { id, size }
    }

    fn new_empty_space(size: usize) -> Self {
        Self { id: u64::MAX, size }
    }

    fn is_empty(&self) -> bool {
        self.id == u64::MAX
    }
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

fn read_filesystem_for_file_size(input: &str) -> Vec<DiskEntry> {
    let line = input.lines()
        .next()
        .expect("Input should not be empty");

    let numbers: Vec<usize> = line.chars()
        .map(|c| c.to_digit(10).expect("Expected digit") as usize)
        .collect();

    let mut disk = Vec::new();
    let mut next_file_id: u64 = 0;

    for chunk in numbers.chunks(2) {
        let file_size = chunk[0];
        disk.push(DiskEntry::new_file(next_file_id, file_size));
        next_file_id += 1;

        if chunk.len() > 1 {
            let space_size = chunk[1];
            disk.push(DiskEntry::new_empty_space(space_size));
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

fn defragment_using_file_size(disk: &mut Vec<DiskEntry>) {
    let mut current_pos = disk.len() - 1;
    
    while current_pos > 0 {
        let current_entry = disk[current_pos];
        
        if current_entry.is_empty() {
            current_pos -= 1;
            continue;
        }

        if let Some(target_pos) = find_suitable_empty_space(disk, current_pos, current_entry.size) {
            move_file_to_space(disk, current_pos, target_pos);
        }
        
        current_pos -= 1;
    }
}

fn find_suitable_empty_space(disk: &[DiskEntry], current_pos: usize, needed_size: usize) -> Option<usize> {
    disk[..current_pos]
        .iter()
        .position(|entry| entry.is_empty() && entry.size >= needed_size)
}

fn move_file_to_space(disk: &mut Vec<DiskEntry>, from_pos: usize, to_pos: usize) {
    let file = disk[from_pos];
    let empty_space_size = disk[to_pos].size;

    disk[to_pos] = DiskEntry::new_file(file.id, file.size);
    disk[from_pos] = DiskEntry::new_empty_space(file.size);

    if file.size < empty_space_size {
        disk.insert(
            to_pos + 1, 
            DiskEntry::new_empty_space(empty_space_size - file.size)
        );
    }
}

fn get_checksum(file_path: &str, part: Part) -> u64 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    if part == Part1 {
        let mut disk = read_filesystem(&file_contents);
        defragment(&mut disk);

        disk.iter()
            .enumerate()
            .filter(|(_, &id)| id != u64::MAX)
            .map(|(i, &id)| id * i as u64)
            .sum()
    } else {
        let mut disk = read_filesystem_for_file_size(&file_contents);
        defragment_using_file_size(&mut disk);

        disk.iter()
            .flat_map(|&disk_entry| (0..disk_entry.size).map(move |_| disk_entry.id))
            .enumerate()
            .map(|(i, id)| if id != u64::MAX { id * i as u64 } else { 0 })
            .sum()
    }
}

fn main() {
    println!("Part 1 checksum: {}", get_checksum("./test.txt", Part1));
    println!("Part 2 checksum: {}", get_checksum("./test.txt", Part2));
}

#[cfg(test)]
mod tests {
    use crate::get_checksum;
    use crate::Part::{Part1, Part2};

    #[test]
    fn returns_expected_value_test_data_for_part_1() {
        let checksum = get_checksum("./test.txt", Part1);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_1() {
        let checksum = get_checksum("./input.txt", Part1);
        assert_eq!(checksum, 6349606724455);
    }

    #[test]
    fn returns_expected_value_test_data_for_part_2() {
        let checksum = get_checksum("./test.txt", Part2);
        assert_eq!(checksum, 2858);
    }

    #[test]
    fn returns_expected_value_for_input_data_for_part_2() {
        let checksum = get_checksum("./input.txt", Part2);
        assert_eq!(checksum, 6376648986651);
    }
}
