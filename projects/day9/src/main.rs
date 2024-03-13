use day9::*;
use utils::*;
fn main() { 
    if let Ok(lines) = read_lines("./input/input.txt") {
        let flattened = lines.flatten();
        //let answer = part_one(flattened);
        let answer = part_two(flattened);
        println!("{answer}");
    }
}
