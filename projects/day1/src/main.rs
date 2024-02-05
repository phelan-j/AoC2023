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

// returns the numeric value of a single char numeric string
// or None otherwise
fn as_numeric_string(value: &str) -> Option<u32> {
    if value.len() == 1 {
        if let Some(c) = value.chars().next() {
            return c.to_digit(10)
        }
    }
    None
}

// checks if a digit string is contained in value at index
// if this would be past the length of the string returns false
fn check_digit_string(value: &String, index: usize, digit: &str) -> bool {
    if digit.len() <= value.len() - index {
        let digit_str = &value[index..index+digit.len()];
        if digit == digit_str {
            return true
        }
    }
    false
}

// for a string calculates the number which has 
// the first digit as the first numeric or verbal
// number representation and the last digit as the 
// last numeric or verbal number representation in 
// the string
fn calibration_value(value: &String) -> u32 {
    let n = value.len();
    let mut found_forwards = false;
    let mut found_backwards = false;
    let mut f = 0;
    let mut b = 0;


    let i_max = n;
    for i in 0..i_max {
        let r = n - i - 1;
        if !found_forwards {
            // TODO: keep forward search if not found backwards string
            if let Some(p) = as_numeric_string(&value[i..=i]) {
                found_forwards = true;
                f = p;
            }
        }
        if !found_backwards {
            if let Some(p) = as_numeric_string(&value[r..=r]) {
                found_backwards = true;
                b = p;
            }
        }
        if !found_forwards || !found_backwards {
            for j in 0..DIGITS.len() {
                let digit = DIGITS[j];
                if !found_forwards && check_digit_string(value, i, digit) {
                    found_forwards = true;
                    f = j as u32;
                }
                if !found_backwards && check_digit_string(value, r, digit) {
                    found_backwards = true;
                    b = j as u32;
                }
                if found_forwards && found_backwards {
                    break;
                }
            }
        }
    }
    10 * f + b
}

