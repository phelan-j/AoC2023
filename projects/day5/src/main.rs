use day5::*;
fn main() { 
    if let Ok(lines) = read_lines("./input/input.txt") {
        let flattened = lines.flatten();
        // part_one(flattened);
        part_two(flattened);
    }
}
