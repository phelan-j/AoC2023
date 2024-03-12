use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Add;
use std::ops::Mul;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn custom_parse_to_vec<T>(line: &str, delimiter: &str, parse: fn(&str) -> T) -> Vec<T> {
    line.split(delimiter).map(parse).collect()
}

pub fn parse_to_vec<T>(line: &str) -> Vec<T> 
    where T : From<u32> + Add<T, Output = T> + Mul<T, Output = T>
{
    parse_to_vec_base::<T>(line, 10)
}

pub fn parse_to_vec_base<T>(line: &str, base: u32) -> Vec<T> 
    where T : From<u32> + Add<T, Output = T> + Mul<T, Output = T>
{
    let mut vec: Vec<T> = Vec::new(); 
    let mut val = None;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(base) {
            let d = T::from(d);
            val = match val {
                Some(v) => Some(T::from(base) * v + d),
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


