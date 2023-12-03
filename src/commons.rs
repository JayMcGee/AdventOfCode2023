use std::fs::read_to_string;

pub fn read_lines(filepath: &str) -> Vec<String> {
    // Opens a file and returns a Vector of Strings containing each line
    let mut result = Vec::new();

    for line in read_to_string(filepath).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}
