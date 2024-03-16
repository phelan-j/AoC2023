const N : u8 = 0b0001;
const E : u8 = 0b0010;
const S : u8 = 0b0100;
const W : u8 = 0b1000;
const MASK : u8 = N | E | S | W;
const DIRECTION : &str = ".NELS|FXWJ-X7";

pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    calculate_path_and_closure::<I>(lines).0
}
        
pub fn part_two<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    calculate_path_and_closure::<I>(lines).1
}

pub fn calculate_path_and_closure<I>(lines: I) -> (i64, i64)
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
    let mut ab: Vec<Vec<i8>> = vec![vec![0; m+1]; n+1];
    
    // before we start looping work out a direction to travel in

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

    let mut l : Vec<(usize, usize)> = Vec::new();
    let mut r : Vec<(usize, usize)> = Vec::new();

    // ab is such that
    // -1 if on path
    // 1 if left of path (assuming clockwise)
    // 2 if right of path (assuming clockwise)
    loop {
        let c = v[j][i];
        ab[j][i] = -1;
        len += 1;
        let f = ((d << 2) | (d >> 2)) & MASK;
        let dn = f ^ c;
        if dn == ((f >> 1) | (f << 3)) & MASK { cw += 1; }
        if dn == ((f << 1) | (f >> 3)) & MASK { cw -= 1; }

        if d == E || dn == E {
            if j > 0 { ab[j - 1][i] |= 1; l.push((i, j - 1)); }
            if j < n { ab[j + 1][i] |= 2; r.push((i, j + 1)); }
        }

        if d == N || dn == N {
            if i > 0 { ab[j][i - 1] |= 1; l.push((i - 1, j)); }
            if i < m { ab[j][i + 1] |= 2; r.push((i + 1, j)); }
        }

        if d == W || dn == W {
            if j > 0 { ab[j - 1][i] |= 2; r.push((i, j - 1)); }
            if j < n { ab[j + 1][i] |= 1; l.push((i, j + 1)); }
        }

        if d == S || dn == S {
            if i > 0 { ab[j][i - 1] |= 2; r.push((i - 1, j)); }
            if i < m { ab[j][i + 1] |= 1; l.push((i + 1, j)); }
        }

        d = dn;

        match d {
            E => i += 1,
            W => i -= 1,
            S => j += 1,
            N => j -= 1,
            _ => panic!("Unknown direction {d:#06b}")
        }
        if i == i_s && j == j_s { break; }
    }



    let mut visited = vec![vec![0; m + 1]; n + 1];
    let mut inside = if cw < 0 { l } else { r };
    let mut inside_count = 0;
    while let Some((i_s,j_s)) = inside.pop() {
        let mut i = i_s;
        let mut j = j_s;
        loop {
            if ab[j][i] >= 0 && visited[j][i] < 1 {
                inside_count += 1;
                visited[j][i] = 1;
                if i > 0 && ab[j][i - 1] >= 0 && visited[j][i - 1] < 1 {
                    i -= 1;
                }
                else if j > 0 && ab[j - 1][i] >= 0 && visited[j - 1][i] < 1 {
                    j -= 1;
                }
                else if i < m && ab[j][i + 1] >= 0 && visited[j][i + 1] < 1 {
                    i += 1;
                }
                else if j < n && ab[j + 1][i] >= 0 && visited[j + 1][i] < 1 {
                    j += 1;
                }
            }
            else { break; }
        }
    }
    (len / 2, inside_count)
}


