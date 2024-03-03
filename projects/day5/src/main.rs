use day5::*;
use std::collections::BTreeMap;
fn main() { 
    if let Ok(lines) = read_lines("./input/input.txt") {
        let flattened = lines.flatten();
        part_one(flattened);
    }
}

fn part_one<I>(mut flattened: I)
    where I : Iterator<Item = String> {
    // load in first line to read seeds
    let line = flattened.next().unwrap();
    // read in vector of seeds - mutable as we will update
    let mut seeds = parse_to_vec(&line);
    // store all maps in a vector
    let mut maps: Vec<BTreeMap<u32, (u32, u32)>> = Vec::new();
    let mut map: BTreeMap<u32, (u32, u32)> = BTreeMap::new();
    for line in flattened {
        let vec = parse_to_vec(&line);
        // if more than 2 values then assume mapping
        if vec.len() > 2 {
            // insert into map, keyed on source
            map.insert(vec[1], (vec[0], vec[2]));
        // if map have entries add to vector and re-init
        } else if map.len() > 0 {
            maps.push(map);
            map = BTreeMap::new();
        }
    }
    // final push
    if map.len() > 0 {
        maps.push(map);
    }
    // loop over seeds
    // store min seed value
    let mut s_min = None;
    for s in seeds.iter_mut() {
        for map in &maps {
            // deference to use in binary search
            let sv = *s;
            let keys: Vec<&u32> = map.keys().clone().collect();
            // binary search valid as keys sorted in BTree
            let index = match keys.binary_search(&&sv) {
                Ok(i) => i,
                // if err - 1 as value is insertion index
                Err(i) => if i > 0 { i - 1 } else { 0 }
            };
            let &source = keys[index];
            let (destination, length) = map[&source];
            if source <= sv && sv < source + length {
                // if within domain, offset to range
                *s = destination + (sv - source);
            }
        }
        // update min
        s_min = match s_min { 
            Some(m) => Some(if s < m { s } else { m }),
            None => Some(s)
        }
    }
    // print min sum
    if let Some(s) = s_min {
        println!("{s}");
    }
}
