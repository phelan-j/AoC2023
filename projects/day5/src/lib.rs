use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_to_vec(line: &str) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    let mut val = None;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(10) {
            val = match val {
                Some(v) => Some(10 * v + d),
                None => Some(d)
            }
        }
        else if let Some(v) = val { 
            vec.push(v);
            val = None;
        }
    }
    if let Some(v) = val { vec.push(v); }
    vec
}
