use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn get_digits(string: &str) -> Vec<u32> {
    let mut digits = Vec::new();

    for c in string.chars() {
        if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap());
        }
    }

    digits
}

fn first_and_last(digits: &Vec<u32>) -> u32 {
    let first: u32 = digits[0];
    let last: u32 = digits[digits.len() - 1];

    (first * 10) + last
}

fn main() -> std::io::Result<()> {
    let lines = read_lines("src/Day1Input.txt");
    let mut numbers: Vec<u32> = Vec::new();

    for line in lines {
        let digits = get_digits(&line);
        let fl = first_and_last(&digits);
        numbers.push(fl);
        println!("{:?}, {}", digits, fl); // For debugging
    }

    let sum: u32 = numbers.iter().sum();
    println!("{}", sum); // For debugging

    Ok(())
}
