use day6::*;
fn main() { 
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut flattened = lines.flatten();
        //let times = parse_to_vec(&flattened.next().unwrap());
        //let distances = parse_to_vec(&flattened.next().unwrap());
        //let answer = part_one(times, distances);
        //
        let time = parse_ignore_space(&flattened.next().unwrap());
        let distance = parse_ignore_space(&flattened.next().unwrap());
        let answer = part_two(time, distance);
        println!("{answer}");
    }
}
