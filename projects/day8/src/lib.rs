use std::collections::HashMap;

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NODE_LABEL_LENGTH : usize = 3;
const START : u32 = 0;
const END : u32 = (ALPHABET.len() as u32).pow(NODE_LABEL_LENGTH as u32) - 1;

pub fn part_one<I>(lines: I) -> u32
    where I : Iterator<Item = String> {
    node_path_count::<I>(lines)
}

pub fn node_path_count<I>(mut lines: I) -> u32
    where I : Iterator<Item = String> {
    let mut nodes : HashMap<u32,(u32,u32)> = HashMap::new();
    let line = lines.next().unwrap();
    let left: Vec<bool> = line.chars().map(|c| match c { 
        'L' => true, 
        'R' => false, 
        c => panic!("unknown instruction {c}") 
    }).collect();
    // skip
    lines.next();
    for line in lines {
        let (k,l,r) = parse_node_line(&line);
        nodes.insert(k, (l,r));
    }
    let mut i : u32 = 0;
    let mut k = START;
    while k != END {
        let d = left[i as usize % left.len()];
        let v = nodes[&k];
        k = if d { v.0 } else { v.1 };
        i += 1
    }
    i
}


fn parse_node_line(line: &str) -> (u32, u32, u32) {
    let a = parse_node(&line[0..NODE_LABEL_LENGTH]);
    let b = parse_node(&line[NODE_LABEL_LENGTH + 4..2 * NODE_LABEL_LENGTH + 4]);
    let c = parse_node(&line[2 * NODE_LABEL_LENGTH + 6..3 * NODE_LABEL_LENGTH + 6]);
    (a,b,c)
}

fn parse_node(s: &str) -> u32 {
    let mut v : u32 = 0;
    let chars = s.chars();
    for c in chars {
        v = v * ALPHABET.len() as u32 + ALPHABET.find(c).unwrap() as u32;
    }
    v
}
