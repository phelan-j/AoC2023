use day3::*;
fn main() {
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut sum = 0;
        let mut lines_flattened = lines.flatten();
        let mut prev_line: Option<String> = None;
        let mut curr_line: Option<String> = lines_flattened.next();
        let mut next_line: Option<String> = lines_flattened.next();

        while let Some(line) = curr_line {
            curr_line = Some(line);
            let valid = sum_gear_ratios(&prev_line, &curr_line, &next_line);
            sum = sum + valid;
            prev_line = curr_line;
            curr_line = next_line;
            next_line = lines_flattened.next();
        }
        let valid = sum_gear_ratios(&prev_line, &curr_line, &next_line);
        sum = sum + valid;
    }
}

// part two
fn sum_gear_ratios(prev_line: &Option<String>, curr_line: &Option<String>, next_line: &Option<String>) -> u32
{
    let mut sum = 0;
    let mut a_val = 0;
    let mut b_val = 0;
    let mut c_val = 0;
    let mut sym_adj = false;
    let mut a_p = '.';
    let mut b_p = '.';
    let mut c_p = '.';
    
    let prev = if let Some(prev) = prev_line { prev } else { "" };
    let curr = if let Some(curr) = curr_line { curr } else { "" };
    let next = if let Some(next) = next_line { next } else { "" };

    let mut a_chars = prev.chars();
    let mut b_chars = next.chars();
    let mut c_chars = curr.chars();

    let mut a_c = if let Some(x) = a_chars.next() { x } else { '.' };
    let mut b_c = if let Some(x) = b_chars.next() { x } else { '.' };
    let mut c_c = if let Some(x) = c_chars.next() { x } else { '.' };

    let mut a_n = if let Some(x) = a_chars.next() { x } else { '.' };
    let mut b_n = if let Some(x) = b_chars.next() { x } else { '.' };
    let mut c_n = if let Some(x) = c_chars.next() { x } else { '.' };

    for i in 0..curr.len() + 1 {

        if let Some(d) = a_c.to_digit(10) { a_val = 10 * a_val + d; }
        if let Some(d) = b_c.to_digit(10) { b_val = 10 * b_val + d; }
        if let Some(d) = c_c.to_digit(10) { c_val = 10 * c_val + d; }
        
        if c_c == '*' {
            // found a potential gear
            let mut adj_count = 0;
            let mut gear_ratio = 1;
            let mut a_temp = if is_digit(a_c) { a_val } else { 0 };
            let mut b_temp = if is_digit(b_c) { b_val } else { 0 };
            a_temp = if let Some(d) = a_n.to_digit(10) { 10 * a_temp + d } else { 0 }; 
            b_temp = if let Some(d) = b_n.to_digit(10) { 10 * b_temp + d } else { 0 };
            let mut c_temp = if let Some(d) = c_n.to_digit(10) { d } else { 0 };
            // keep reading digits until we find no more
            if a_temp > 0 {
                let a_str = &prev[i+2..];
                let mut a_clone = a_str.chars();
                while let Some(n) = a_clone.next() { if let Some(d) = n.to_digit(10) { a_temp = 10 * a_temp + d; } else { break; } }
            }
            if b_temp > 0 {
                let b_str = &next[i+2..];
                let mut b_clone = b_str.chars();
                while let Some(n) = b_clone.next() { if let Some(d) = n.to_digit(10) { b_temp = 10 * b_temp + d; } else { break; } }
            }
            if c_temp > 0 {
                let mut c_clone = curr[i+2..].chars();
                while let Some(n) = c_clone.next() { if let Some(d) = n.to_digit(10) { c_temp = 10 * c_temp + d; } else { break; } }
            }
            if !is_digit(a_c) {
                if a_val > 0 {
                    gear_ratio = gear_ratio * a_val;
                    adj_count = adj_count + 1;
                }
                if a_temp > 0 {
                    gear_ratio = gear_ratio * a_temp;
                    adj_count = adj_count + 1;
                }
            }
            else {
                if a_temp > 0 {
                    gear_ratio = gear_ratio * a_temp;
                    adj_count = adj_count + 1;
                }
                else if a_val > 0 {
                    gear_ratio = gear_ratio * a_val;
                    adj_count = adj_count + 1;
                }
            }


            if !is_digit(b_c) {
                if b_val > 0 {
                    gear_ratio = gear_ratio * b_val;
                    adj_count = adj_count + 1;
                }
                if b_temp > 0 {
                    gear_ratio = gear_ratio * b_temp;
                    adj_count = adj_count + 1;
                }
            }
            else {
                if b_temp > 0 {
                    gear_ratio = gear_ratio * b_temp;
                    adj_count = adj_count + 1;
                }
                else if b_val > 0 {
                    gear_ratio = gear_ratio * b_val;
                    adj_count = adj_count + 1;
                }
            }
            if c_val > 0 {
                gear_ratio = gear_ratio * c_val;
                adj_count = adj_count + 1;
            }
            if c_temp > 0 {
                gear_ratio = gear_ratio * c_temp;
                adj_count = adj_count + 1;
            }
            if adj_count == 2 { 
                sum = sum + gear_ratio;
            }
        }
        if !is_digit(a_c) { a_val = 0; }
        if !is_digit(b_c) { b_val = 0; }
        if !is_digit(c_c) { c_val = 0; }

        a_p = a_c;
        a_c = a_n;
        a_n = if let Some(a_n) = a_chars.next() { a_n } else { '.' };

        b_p = b_c;
        b_c = b_n;
        b_n = if let Some(b_n) = b_chars.next() { b_n } else { '.' };

        c_p = c_c;
        c_c = c_n;
        c_n = if let Some(c_n) = c_chars.next() { c_n } else { '.' };
    }
    sum
}
// part one
fn sum_part_numbers(prev_line: &Option<String>, curr_line: &Option<String>, next_line: &Option<String>) -> u32
{
    let mut sum = 0;
    let mut val = 0;

    let mut sym_adj = false;
    // above
    let mut a_p = '.';
    // below
    let mut b_p = '.';
    // current
    let mut c_p = '.';
    
    let prev = if let Some(prev) = prev_line { prev } else { "" };
    let curr = if let Some(curr) = curr_line { curr } else { "" };
    let next = if let Some(next) = next_line { next } else { "" };

    let mut a_chars = prev.chars();
    let mut b_chars = next.chars();
    let mut c_chars = curr.chars();

    let mut a_c = if let Some(x) = a_chars.next() { x } else { '.' };
    let mut b_c = if let Some(x) = b_chars.next() { x } else { '.' };
    let mut c_c = if let Some(x) = c_chars.next() { x } else { '.' };

    let mut a_n = if let Some(x) = a_chars.next() { x } else { '.' };
    let mut b_n = if let Some(x) = b_chars.next() { x } else { '.' };
    let mut c_n = if let Some(x) = c_chars.next() { x } else { '.' };

    for i in 0..curr.len() + 1 {

        // found a digit
        if let Some(d) = c_c.to_digit(10) {
            if is_symbol(a_p) || is_symbol(b_p) || is_symbol(a_c) || is_symbol(b_c) || is_symbol(c_p) {
                sym_adj = true;
            }
            val = 10 * val + d;
        }
        // found a blank
        else 
        {
            if is_symbol(a_c) || is_symbol(b_c) || is_symbol(c_c) {
                sym_adj = true;
            }
            if val > 0 && sym_adj {
                sum = sum + val;
            }
            val = 0;
            if !is_symbol(a_c) && !is_symbol(b_c) && !is_symbol(c_c) {
                sym_adj = false;
            }
        }

        a_p = a_c;
        a_c = a_n;
        a_n = if let Some(a_n) = a_chars.next() { a_n } else { '.' };

        b_p = b_c;
        b_c = b_n;
        b_n = if let Some(b_n) = b_chars.next() { b_n } else { '.' };

        c_p = c_c;
        c_c = c_n;
        c_n = if let Some(c_n) = c_chars.next() { c_n } else { '.' };
    }
    sum
}
fn is_digit(c: char) -> bool { c >= '0' && c <= '9' }
fn is_symbol(c: char) -> bool {
    c != '.' && (c < '0' || c > '9')
}
