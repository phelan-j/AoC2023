pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    galaxy_distances::<I>(lines, 1)
}

pub fn part_two<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    galaxy_distances::<I>(lines, 999999)
}


pub fn galaxy_distances<I>(lines: I, expansion: i64) -> i64
    where I : Iterator<Item = String> {

    let mut row_counts : Vec<i64> = Vec::new();
    let mut col_counts : Vec<i64> = Vec::new();

    let mut points : Vec<(i64, i64)> = Vec::new();

    for (j, line) in lines.enumerate() {
        let mut row_count = 0;
        for (i, c) in line.chars().enumerate() {
            let count = if c == '#' { 1 } else { 0 };
            if count > 0 { points.push((i as i64,j as i64)); }
            row_count += count;
            if i < col_counts.len() { col_counts[i] += count; }
            else { col_counts.push(count); }
        }
        row_counts.push(row_count);
    }
    let mut row_offsets : Vec<i64> = Vec::new();
    let mut col_offsets : Vec<i64> = Vec::new();
    let mut o = 0;
    for &v in &row_counts {
        row_offsets.push(o);
        if v == 0 { o += 1; }
    }

    let mut o = 0;
    for &v in &col_counts {
        col_offsets.push(o);
        if v == 0 { o += 1; }
    }

    for (i,j) in points.iter_mut() {
        *i += expansion * col_offsets[*i as usize];
        *j += expansion * row_offsets[*j as usize];
    }

    let mut sum = 0;
    while let Some((i1,j1)) = points.pop() {
        for &(i2,j2) in points.iter() {
            let id = if i1 < i2 { i2 - i1 } else { i1 - i2 };
            let jd = if j1 < j2 { j2 - j1 } else { j1 - j2 };
            sum += id as i64 + jd as i64;

        }
    }
    sum
}
 
