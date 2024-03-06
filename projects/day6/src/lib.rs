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
            let d = d as u32;
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

pub fn part_one(times: Vec<u32>, distances: Vec<u32>) -> u32 {
    let mut prd = 1;
    for i in 0..times.len() {
        let n = times[i];
        let r = distances[i];
        let x = ((n * n - 4 * r) as f64).sqrt();
        let x = 0.5 * (n as f64 + x);
        let x = 2 * (x.ceil() as u32) - n - 1;
        prd *= x;
    }
    prd
}

