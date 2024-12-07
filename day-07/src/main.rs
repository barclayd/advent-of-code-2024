use std::fs;

fn get_value(file_path: &str) -> i32 {
    let file_contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
   8
}


fn main() {
    println!("Value: {}", get_value("./input.txt"));
}

#[cfg(test)]
mod tests {
    use crate::get_value;

    #[test]
    fn returns_expected_value_test_data() {
        let value = get_value("./test.txt");
        assert_eq!(value, 8);
    }

    #[test]
    fn returns_expected_value_for_input_data() {
        let value = get_value("./input.txt");
        assert_eq!(value, 8);
    }
}