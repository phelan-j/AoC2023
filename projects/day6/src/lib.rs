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

pub fn parse_ignore_space(line: &str) -> f64
{
    let mut val = None;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(10) {
            let d = d as f64;
            val = match val {
                Some(v) => Some(10.0 * v + d),
                None => Some(d)
            }
        }
    }
    match val {
        Some(v) => v,
        None => 0.0
    }
}




pub fn part_one(times: Vec<f64>, distances: Vec<f64>) -> f64 {
    let mut prd : f64 = 1.0;
    for i in 0..times.len() {
        let x = part_two(times[i], distances[i]);
        prd *= x;
    }
    prd
}

pub fn part_two(n: f64, r: f64) -> f64 {
    2.0 * (0.5 * (n + (n * n - 4.0 * r).sqrt())).ceil() - n - 1.0
}
