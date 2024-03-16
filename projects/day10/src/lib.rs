const N : u8 = 0b0001;
const E : u8 = 0b0010;
const S : u8 = 0b0100;
const W : u8 = 0b1000;
const NE : u8 = N | E;
const NW : u8 = N | W;
const SE : u8 = S | E;
const SW : u8 = S | W;
const NS : u8 = N | S;
const WE : u8 = W | E;
const MASK : u8 = N | E | S | W;


const DIRECTION : &str = ".NELS|FXWJ-X7";

fn debug_print(v: u8) -> String  {
    let v = v as usize;
    DIRECTION[v..=v].to_string()
}

pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    let mut i_s = 0;
    let mut j_s = 0;
    let mut v: Vec<Vec<u8>> = Vec::new();
    let mut ab: Vec<Vec<i8>> = Vec::new();
    println!("input");
    for (j,line) in lines.enumerate() {
        println!("{line}");
        v.push(line.chars().enumerate().map(|(i,c)| 
            if c == 'S' { i_s = i; j_s = j; 0 }
            else if let Some(k) = &DIRECTION.find(c) { *k as u8 }
            else { panic!("Unknown symbol {c}") }
        ).collect());
        ab.push(vec![0; line.len()]);
    }
    let mut i = i_s;
    let mut j = j_s;
    let mut d = 0;
    let mut len = 0;
    let m = v[0].len() - 1;
    let n = v.len() - 1;
    
    // before we start looping work out a direction to travel in

    let mut s = 0;
    if i > 0 && v[j][i - 1] & E != 0 {
        println!("v[j][i - 1] = {:#06b}",v[j][i - 1]);
        s |= W;
    }
    if j > 0 && v[j - 1][i] & S != 0 {
        println!("v[j - 1][i] = {:#06b}",v[j - 1][i]);
        s |= N;
    }
    if i < m - 1 && v[j][i + 1] & W != 0 {
        println!("v[j][i + 1] = {:#06b}",v[j][i + 1]);
        s |= E;
    }
    if j < n - 1 && v[j + 1][i] & N != 0 {
        println!("v[j + 1][i] = {:#06b}",v[j + 1][i]);
        s |= S;
    }
    println!("s: {s:#06b}");
    v[j][i] = s;

    if s & E != 0 { d = W; }
    else if s & W != 0 { d = E; }
    else if s & S != 0 { d = N; }
    else if s & N != 0 { d = S; };

    println!("d: {d:#06b}");
    
    let mut cw = 0i64;

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
        println!("c: {c:#06b}, i: {i}, j: {j}, d: {d:#06b}, f: {f:#06b}, dn: {dn:#06b}");

        if d == E || dn == E {
            if j > 0 { ab[j - 1][i] |= 1; }
            if j < n - 1 { ab[j + 1][i] |= 2; }
        }

        if d == N || dn == N {
            if i > 0 { ab[j][i - 1] |= 1; }
            if i < m - 1 { ab[j][i + 1] |= 2; }
        }

        if d == W || dn == W {
            if j > 0 { ab[j - 1][i] |= 2; }
            if j < n - 1 { ab[j + 1][i] |= 1; }
        }

        if d == S || dn == S {
            if i > 0 { ab[j][i - 1] |= 2; }
            if i < m - 1 { ab[j][i + 1] |= 1; }
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

    println!("cw: {cw}");

    println!("parsed:");
    for j in 0..ab.len() {
        let s : String = ab[j].iter().map(|&t| 
            if t < 0 { 'P' }
            else if t == 1 { 'L' }
            else if t == 2 { 'R' }
            else { '.' }
        ).collect();
        println!("{s}");

    }
    len / 2
}


