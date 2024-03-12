use std::collections::HashMap;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_LENGTH : usize = ALPHABET.len() as usize;
const NODE_LABEL_LENGTH : usize = 3;
const START : usize = 0;
const END : usize = ALPHABET_LENGTH.pow(NODE_LABEL_LENGTH as u32) - 1;

pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    node_path_count::<I>(lines, |x| x == START, |x| x == END)
}

pub fn part_two<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    node_path_count::<I>(lines, |x| x % ALPHABET_LENGTH == START, |x| (x + 1) % ALPHABET_LENGTH == 0)
}


pub fn node_path_count<I>(mut lines: I, is_start: fn(usize) -> bool, is_end: fn(usize) -> bool) -> i64
    where I : Iterator<Item = String> {
    // store map from node to n child nodes
    let mut nodes : HashMap<usize,Vec<usize>> = HashMap::new();
    let line = lines.next().unwrap();
    // only L and R specified, but solution works for n child nodes
    let indices: Vec<usize> = line.chars().map(|c| match c { 
        'L' => 0usize, 
        'R' => 1usize, 
        c => panic!("unknown instruction {c}") 
    }).collect();
    // skip a line
    lines.next();
    let mut paths : Vec<usize> = Vec::new();
    for line in lines {
        let vec = parse_node_line(&line);
        // first node mapes to list of children
        let k = vec[0];
        // solution generic for n child nodes
        nodes.insert(k, vec[1..].to_vec()); 
        // mark this node as entry point to find cycle
        if is_start(k) {
            paths.push(k);
        }
    }
    // c_max stores the last index at which a cycle starts 
    // across all starting points
    let mut c_max = 0usize;
    let mut cycles : Vec<(usize,usize,Vec<usize>)> = Vec::new();
    for &s in &paths {
        // store cycles and flagged end points and calcuate max cycle start
        let (c,n,e) = detect_cycle(s, &nodes, &indices, is_end);
        cycles.push((c,n,e));
        if c > c_max { c_max = c; }
    }
    let mut n : i64 = 1;
    let mut e_m : Vec<i64> = Vec::new();
    e_m.push(0);
    for (c,m,e) in cycles {
        let mut e_n : Vec<i64> = Vec::new();
        // rebase the cycle such that all cycles start at the same point
        let c_diff = c_max - c;
        // loop over existing solutions 
        for a in e {
            let a = if a < c_diff {
                m + a - c_diff
            }
            else {
                a - c_diff
            };
            for &b in &e_m {
                // for each flagged end point in a cycle calculate the potential solution
                // x mod lcm(m,n) === a mod m === b mod n
                // exists if a - b === 0 mod gcd(m,n)
                if let Some(x) = chinese_rem(a as i64,m as i64,b as i64,n as i64) {
                    e_n.push(x);
                }
            }
        }
        e_m = e_n;
        let (g,_,_) = extended_euclid(m as i64,n);
        n = (m as i64) * n / g;
    }
    let mut e_min : Option<i64> = None;
    for e in e_m {
        let e = if e < 0 { n + e } else { e };
        e_min = Some(match e_min {
            Some(m) => if e < m { e } else { m },
            None => e
        });
    }
    match e_min {
        Some(e) => e,
        None => 0
    }
}

// extended euclidean algorithm, producing gcd and Bezout coefficients
fn extended_euclid(a: i64, b: i64) -> (i64,i64,i64) {
    let (mut r_p, mut s_p, mut t_p) = (a, 1, 0);
    let (mut r_c, mut s_c, mut t_c) = (b, 0, 1);
    while r_c != 0 {
        let q = r_p / r_c;
        let (r_n, s_n, t_n) =
        (r_p - q * r_c, s_p - q * s_c, t_p - q * t_c);
        (r_p, s_p, t_p) = (r_c, s_c, t_c);
        (r_c, s_c, t_c) = (r_n, s_n, t_n);
    }
    (r_p,s_p,t_p)
}

// calculates x such that x === a mod m === b mod n
// N.B. for g = gcd(m,n), M = lcm(m,n) = mn / g
// a unique solution modulo M exists only if a === b mod g
fn chinese_rem(a: i64, m: i64, b: i64, n: i64) -> Option<i64> {
    let a = a % m;
    let b = b % n;
    // reduce magnitude of a mod m, b mod n
    let a = if m - a < a { a - m } else { a };
    let b = if n - b < b { b - n } else { b };
    let (g,s,t) = extended_euclid(m,n);
    if (a - b) % g != 0 { 
        None
    }
    else {
        let x = a * t * n / g + b * s * m / g;
        Some(x)
    }
}

// given a map from node x to nodes y_k and string of 
// k indices to travel at each node (repeated) 
// detect the length of the cycle
//
// record nodes which are flagged using boolean function
// only if they occur in cycle, relative to cycle start
fn detect_cycle(start: usize, map : &HashMap<usize, Vec<usize>>, dir : &Vec<usize>, flag: fn(usize) -> bool) -> (usize, usize, Vec<usize>) {
    let mut visited : HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    let mut i : usize = 0;
    let mut node : usize = start;
    let mut end_nodes : Vec<usize> = Vec::new();
    loop {
        let j = i % dir.len();
        let d = dir[j];
        if flag(node) {
            end_nodes.push(i);
        }
        let child = map[&node][d];
        let visited_at = visited.entry(node).or_insert(HashMap::new());
        let &mut p = visited_at.entry(j).or_insert(i);
        if p < i {
            let n = i - p;
            let mut end_nodes_cycle : Vec<usize> = Vec::new();
            for e in end_nodes {
                if e >= p {
                    end_nodes_cycle.push(e - p);
                }
            }
            return (p, n, end_nodes_cycle)
        }
        node = child;
        i += 1;
    }
}

// parse alphabetic strings to integer based on alphabet
fn parse_node_line(line: &str) -> Vec<usize> {
    let mut vec : Vec<usize> = Vec::new();
    let mut val : Option<usize> = None;
    for c in line.chars() {
        if let Some(i) = ALPHABET.find(c) {
            val = Some(match val {
                Some(v) => v * ALPHABET_LENGTH + i,
                None => i
            });
        }
        else if let Some(v) = val {
            vec.push(v);
            val = None;
        }
    }
    if let Some(v) = val { vec.push(v); }
    vec
}
