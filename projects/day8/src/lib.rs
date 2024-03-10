use std::collections::HashMap;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHABET_LENGTH : usize = ALPHABET.len() as usize;
const NODE_LABEL_LENGTH : usize = 3;
const START : usize = 0;
const END : usize = ALPHABET_LENGTH.pow(NODE_LABEL_LENGTH as u32) - 1;

pub fn part_one<I>(lines: I) -> u64
    where I : Iterator<Item = String> {
    node_path_count::<I>(lines, |x| x == START, |x| x == END)
}

pub fn part_two<I>(lines: I) -> u64
    where I : Iterator<Item = String> {
    node_path_count::<I>(lines, |x| x % ALPHABET_LENGTH == START, |x| x % ALPHABET_LENGTH == ALPHABET_LENGTH - 1)
}


pub fn node_path_count<I>(mut lines: I, is_start: fn(usize) -> bool, is_end: fn(usize) -> bool) -> u64
    where I : Iterator<Item = String> {
    let mut nodes : HashMap<usize,Vec<usize>> = HashMap::new();
    let line = lines.next().unwrap();
    let child: Vec<usize> = line.chars().map(|c| match c { 
        'L' => 0usize, 
        'R' => 1usize, 
        c => panic!("unknown instruction {c}") 
    }).collect();
    // skip
    lines.next();
    let mut paths : Vec<usize> = Vec::new();
    for line in lines {
        let vec = parse_node_line(&line);
        let k = vec[0];
        nodes.insert(k, vec[1..].to_vec()); 
        if is_start(k) {
            paths.push(k);
        }
    }
    let mut i : u64 = 0;
    loop {
        let j = (i % (child.len() as u64)) as usize;
        let d = child[j];
        let mut b = true;
        for p in &mut paths {
            *p = nodes[p][d];
            b = b && is_end(*p);
        }
        i += 1;
        if b { break; }
    }
    i
}


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
