use day4::*;
use std::collections::HashSet;
fn main() {
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut sum = 0;
        let mut winning: HashSet<u32> = HashSet::new();
        for line in lines.flatten() {
            let card_val = calculate_scratch_card_value(&line, &mut winning);
            sum += card_val;
        }
        println!("{sum}");
    }
}
fn calculate_scratch_card_value(line: &str, winning: &mut HashSet<u32>) -> u32 {
    let mut chars = line.chars();
    let mut val = 0;
    let mut match_count = 0;
    winning.clear();
    while let Some(c) = chars.next() {
        if c == ':' {
            break;
        }
    }
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(10) { val = 10 * val + d; }
        else if c == '|' { 
            if val > 0 { winning.insert(val); }
            val = 0;
            break;
        }
        else { 
            if val > 0 { winning.insert(val); }
            val = 0;
        }
    }
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(10) { val = 10 * val + d; }
        else {
            if winning.contains(&val) { match_count += 1; }
            val = 0;
        }
    }
    if winning.contains(&val) { match_count += 1; }
    if match_count > 0 { 1 << (match_count - 1) } else { 0 }
}
