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
            let valid = process_line(&prev_line, &curr_line, &next_line);
            sum = sum + valid;

            prev_line = curr_line;
            curr_line = next_line;
            next_line = lines_flattened.next();
        }
        let valid = process_line(&prev_line, &curr_line, &next_line);
        sum = sum + valid;
        println!("{sum}");
    }
}

fn process_line(prev_line: &Option<String>, curr_line: &Option<String>, next_line: &Option<String>) -> u32
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
fn is_symbol(c: char) -> bool {
    c != '.' && (c < '0' || c > '9')
}
