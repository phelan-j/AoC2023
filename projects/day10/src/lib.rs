use std::collections::HashSet;

// bit values are important such that
// we can find reverse direction with
// bitwise operations
const N : u8 = 0b0001;
const E : u8 = 0b0010;
const S : u8 = 0b0100;
const W : u8 = 0b1000;
const MASK : u8 = N | E | S | W;
const DIRECTION : &str = ".NELS|FXWJ-X7";

pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    calculate_path_and_closure::<I>(lines).0 / 2
}
        
pub fn part_two<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    calculate_path_and_closure::<I>(lines).1
}

fn calculate_path_and_closure<I>(lines: I) -> (i64, i64)
    where I : Iterator<Item = String> {
    let mut i_s = 0;
    let mut j_s = 0;
    let mut v: Vec<Vec<u8>> = Vec::new();
    for (j,line) in lines.enumerate() {
        v.push(line.chars().enumerate().map(|(i,c)| 
            if c == 'S' { i_s = i; j_s = j; 0 }
            else if let Some(k) = &DIRECTION.find(c) { *k as u8 }
            else { panic!("Unknown symbol {c}") }
        ).collect());
    }
    let mut i = i_s;
    let mut j = j_s;
    let mut d = 0;
    let mut len = 0;
    let m = v[0].len() - 1;
    let n = v.len() - 1;
    
    // before we start looping work out a direction to travel in
    // we don't know (yet) if we are travelling clockwise or anticlockwise
    let mut s = 0;
    if i > 0 && v[j][i - 1] & E != 0 {
        s |= W;
    }
    if j > 0 && v[j - 1][i] & S != 0 {
        s |= N;
    }
    if i < m - 1 && v[j][i + 1] & W != 0 {
        s |= E;
    }
    if j < n - 1 && v[j + 1][i] & N != 0 {
        s |= S;
    }
    v[j][i] = s;

    if s & E != 0 { d = W; }
    else if s & W != 0 { d = E; }
    else if s & S != 0 { d = N; }
    else if s & N != 0 { d = S; };
    
    let mut cw = 0i64;

    let mut l : HashSet<(usize, usize)> = HashSet::new();
    let mut r : HashSet<(usize, usize)> = HashSet::new();
    let mut p : HashSet<(usize, usize)> = HashSet::new();

    loop {
        let c = v[j][i];
        // if we're on the path remove from left and right
        // store path points in p
        p.insert((i,j));
        l.remove(&(i,j));
        r.remove(&(i,j));

        len += 1;
        // work out direction we came from
        let f = ((d << 2) | (d >> 2)) & MASK;
        // work out the next direction based on the pipe section
        let dn = f ^ c;
        // count the number of left and right turns
        // to work out if we are travelling clockwise or not
        if dn == ((f >> 1) | (f << 3)) & MASK { cw += 1; }
        if dn == ((f << 1) | (f >> 3)) & MASK { cw -= 1; }

        // add points on left and right based on our direction
        if d == E || dn == E {
            if j > 0 { l.insert((i, j - 1)); }
            if j < n { r.insert((i, j + 1)); }
        }

        if d == N || dn == N {
            if i > 0 { l.insert((i - 1, j)); }
            if i < m { r.insert((i + 1, j)); }
        }

        if d == W || dn == W {
            if j > 0 { r.insert((i, j - 1)); }
            if j < n { l.insert((i, j + 1)); }
        }

        if d == S || dn == S {
            if i > 0 { r.insert((i - 1, j)); }
            if i < m { l.insert((i + 1, j)); }
        }

        d = dn;

        // increment i or j based on current direction
        match d {
            E => i += 1,
            W => i -= 1,
            S => j += 1,
            N => j -= 1,
            _ => panic!("Disallowed direction {d:#06b}")
        }
        // if we arrived back at the start we're done
        if i == i_s && j == j_s { break; }
    }

    // stored visited points
    let mut vis : HashSet<(usize, usize)> = HashSet::new();
    // if we're going clockwise then we want points on the right
    // else we want points on the left
    let ins = if cw < 0 { l } else { r };
    let mut inside_count = 0;
    for (i_s, j_s) in ins {
        let mut i = i_s;
        let mut j = j_s;
        loop {
            // if we're not on the path and haven't been visited then explore
            if !p.contains(&(i,j)) && !vis.contains(&(i,j)) {
                // count the points inside and track where we've visited
                inside_count += 1;
                vis.insert((i,j));
                // explore left, right, up or down
                if i > 0 && !p.contains(&(i - 1, j)) && !vis.contains(&(i - 1, j)) {
                    i -= 1;
                }
                else if j > 0 && !p.contains(&(i, j - 1)) && !vis.contains(&(i, j - 1)) {
                    j -= 1;
                }
                else if i < m && !p.contains(&(i + 1, j)) && !vis.contains(&(i + 1, j)) {
                    i += 1;
                }
                else if j < n && !p.contains(&(i, j + 1)) && !vis.contains(&(i, j + 1)) {
                    j += 1;
                }
            }
            else { break; }
        }
    }
    (len, inside_count)
}
