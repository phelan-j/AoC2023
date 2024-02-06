use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut sum = 0;
        for line in lines.flatten() {
            let valid = process_line(&line, 12, 13, 14);
            sum = sum + valid;
        }
        println!("{sum}");
    }
}

fn process_line(line: &String, r_lim: u32, g_lim: u32, b_lim: u32) -> u32 {
    if let Some(i_colon) = line.find(":") {
        let id_str = &line["Game ".len()..i_colon];
        if let Ok(id) = id_str.parse::<u32>() {
            let sets = line[i_colon+1..].split(";");
            for set in sets {
                let (r,g,b) = process_set(set);
                if r > r_lim { return 0 }
                if g > g_lim { return 0 }
                if b > b_lim { return 0 }
            }
            return id
        }
    }
    0
}

fn process_set(set: &str) -> (u32, u32, u32) {
    let (mut r, mut g, mut b) = (0,0,0);
    let colors = set.split(",");
    for color in colors {
        let color_trimmed = color.trim();
        if let Some(i_space) = color_trimmed.find(" ") {
            let count_str = color_trimmed[0..=i_space].trim();
            let color_str = color_trimmed[i_space..].trim();
            if let Ok(count) = count_str.parse::<u32>() {
                match color_str {
                    "red" => r = count,
                    "green" => g = count,
                    "blue" => b = count,
                    _ => {}
                }
            }
        }
    }
    (r,g,b)
}
