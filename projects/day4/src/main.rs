use day4::*;
use std::collections::HashSet;
use std::collections::LinkedList;
fn main() {
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut sum = 0;
        let mut winning: HashSet<u32> = HashSet::new();
        let mut mult_list: LinkedList<u32> = LinkedList::new();
        for line in lines.flatten() {
            // part one
            //let card_val = calculate_scratch_card_points(&line, &mut winning);
            //sum += card_val;
            
            // part two
            let mut matches = calculate_scratch_card_matches(&line, &mut winning);
            let count = if let Some(first) = mult_list.pop_front() { first + 1 } else { 1 };
            sum = sum + count;

            if matches > 0 { 
                for item in mult_list.iter_mut() {
                    *item += count;
                    matches -= 1;
                    if matches <= 0 { break; }
                }
                for _i in 0..matches {
                    mult_list.push_back(count);
                }
            }
        }
        println!("{sum}");
    }
}

fn calculate_scratch_card_points(line: &str, winning: &mut HashSet<u32>) -> u32 {
    let matches = calculate_scratch_card_matches(line, winning);
    if matches > 0 { 1 << (matches - 1) } else { 0 }
}
fn calculate_scratch_card_matches(line: &str, winning: &mut HashSet<u32>) -> u32 {
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
    if val > 0 { winning.insert(val); }
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(10) { val = 10 * val + d; }
        else {
            if winning.contains(&val) { match_count += 1; }
            val = 0;
        }
    }
    if val > 0 && winning.contains(&val) { match_count += 1; }
    match_count
}
