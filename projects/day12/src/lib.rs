pub fn part_one<I>(lines: I) -> usize
    where I : Iterator<Item = String> {
        let mut sum = 0;
        for line in lines {
            let a = get_possible_arrangements(&line);
            sum += a;
        }
        sum
}

pub fn part_two<I>(lines: I) -> usize
    where I : Iterator<Item = String> {
        let mut sum = 0;
        for line in lines {
            let unfolded = unfold_line(&line, 5);
            let a = get_possible_arrangements(&unfolded);
            sum += a;
        }
        sum
}


fn unfold_line(line: &str, n: usize) -> String {
    let i = line.find(" ").unwrap();
    let springs = &line[0..i];
    let groups = &line[i+1..];
    let unfolded_springs = vec![springs; n];
    let unfolded_groups = vec![groups; n];
    let unfolded_springs = unfolded_springs.join(",");
    let unfolded_groups = unfolded_groups.join(",");
    unfolded_springs + " " + &unfolded_groups
}


fn get_possible_arrangements(line: &str) -> usize {
    let i = line.find(" ").unwrap();
    let springs = &line[0..i];
    let mut group_sum = 0;
    let mut groups : Vec<usize> = Vec::new();
    for s in line[i+1..].split(",") {
        let g = s.parse::<usize>().unwrap();
        group_sum += g;
        groups.push(g)
    }
    // need at least n - 1 gaps of length at least 1 between contiguous regions
    let min_springs = group_sum + groups.len() - 1;
    calculate_combos(&springs, &groups, min_springs)
}


fn calculate_combos(springs: &str, groups: &[usize], min_springs: usize) -> usize {
    let mut sum = 0;
    let n = springs.len();
    let spring_chars : Vec<char> = springs.chars().collect();
    if groups.len() < 1 {
        for c in spring_chars {
            if c == '#' { return 0 }
        }
        return 1
    }
    if n < 1 || min_springs < 1 || groups.len() < 1 { return 1 }
    if n < min_springs { return 0 }
    let g_s = groups[0];
    let min_springs_next = if groups.len() < 2 { 0 } else { min_springs - g_s - 1 };
    for i_s in 0..=n - min_springs {
        let mut g = g_s;
        let mut i = i_s;
        // loop g times
        while g > 0 && i < n && spring_chars[i] != '.' {
            g -= 1;
            i += 1;
        }
        // we need a possible gap of at least one
        if g == 0 
        {
            if i < n && spring_chars[i] != '#' {
                let r = calculate_combos(&springs[i+1..], &groups[1..], min_springs_next);
                sum += r;
            }
            else if i == n 
            { 
                sum += 1; 
            }
        }
        if spring_chars[i_s] == '#' { break; }
    }
    sum
}

#[cfg(test)]
mod test;
