use day10::*;
use utils::*;
fn main() { 
    if let Ok(lines) = read_lines("./input/input.txt") {
        let flattened = lines.flatten();
        let answer = part_one(flattened);
        println!("{answer}");
    }
}