use std::convert::TryFrom;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

fn get_spelled_digits(string: &str) -> (u32, u32) {
    let spelled_digits: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut digit: u32 = 0;
    let mut len: u32 = 0;

    for (i, sd) in spelled_digits.iter().enumerate() {
        if string.starts_with(sd) {
            digit = u32::try_from(i).unwrap();
            len = u32::try_from(sd.len()).unwrap();
        }
    }

    (digit, len)
}

fn get_digits(string: &str) -> Vec<u32> {
    let mut digits = Vec::new();

    for (i, c) in string.chars().enumerate() {
        if c.is_digit(10) {
            digits.push(c.to_digit(10).unwrap());
        } else {
            // Grab this character and the next five one and check if it's a spelled digit
            let mut last_idx = i + 5;
            if last_idx > string.len() {
                last_idx = string.len()
            }

            let sub_string = &string[i..last_idx];

            let x = get_spelled_digits(sub_string);
            if x.0 != 0 {
                digits.push(x.0);
            }
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
    let mut summation: u32 = 0;

    for line in lines {
        let digits = get_digits(&line);
        let fl = first_and_last(&digits);
        numbers.push(fl);
        println!("{}, {}", line, fl); // For debugging
        summation += fl;
    }

    let sum: u32 = numbers.iter().sum();
    println!("The Sum Is: {} : {}", sum, summation);
    Ok(())
}
