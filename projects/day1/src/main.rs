use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIGITS: &'static [&'static str] = &["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./inputs/input.txt") {
        for line in lines.flatten() {
            let calib_value = calibration_value(&line);
            sum = sum + calib_value;
        }
    }
    println!("{sum}");
}

fn calibration_value(value: &String) -> u32 {
    let first_digit = first_number(&value);
    let last_digit = last_number(&value);
    10 * first_digit + last_digit
}

fn is_numeric(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_numeric_string(value: &str) -> bool {
    if value.len() == 1 {
        if let Some(c) = value.chars().next() {
            return is_numeric(c)
        }
    }
    false
}

fn first_number(value: &String) -> u32 {
    let n = value.len();
    for i in 0..n {
        let chr_str = &value[i..=i];
        if is_numeric_string(chr_str) {
            return chr_str.parse::<u32>().unwrap();
        }
        else {
            for j in 0..DIGITS.len() {
                let digit = DIGITS[j];
                if digit.len() <= n - i {
                    let digit_str = &value[i..i+digit.len()];
                    if digit == digit_str {
                        return j as u32
                    }
                }
            }
        }
    }
    0
}


fn last_number(value: &String) -> u32 {
    let n = value.len();
    for r in 0..n {
        let i = n - r - 1;
        let chr_str = &value[i..=i];
        if is_numeric_string(chr_str) {
            return chr_str.parse::<u32>().unwrap();
        }
        else {
            for j in 0..DIGITS.len() {
                let digit = DIGITS[j];
                if digit.len() <= n - i {
                    let digit_str = &value[i..i+digit.len()];
                    if digit == digit_str {
                        return j as u32
                    }
                }
            }
        }
    }
    0
}
