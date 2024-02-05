use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let first_chr = first_numeric(&value);
    let last_chr = last_numeric(&value);
    let first_parse = first_chr.unwrap().to_string().parse::<u32>().unwrap();
    let last_parse = last_chr.unwrap().to_string().parse::<u32>().unwrap();
    10 * first_parse + last_parse
}

fn is_numeric(c: char) -> bool {
    c >= '0' && c <= '9'
}
fn first_numeric(value: &String) -> Option<char> {
    return value.chars()
        .filter(|&c| is_numeric(c))
        .next()
}
fn last_numeric(value: &String) -> Option<char> {
    return value.chars()
        .rev()
        .filter(|&c| is_numeric(c))
        .next()
}
