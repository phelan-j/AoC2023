use std::cmp::Ordering;

const CARD_ORDER : &str = "23456789TJQKA";
const HAND_SIZE : usize = 5;
const VALUE_SIZE : usize = HAND_SIZE + 1;

pub fn part_one<I>(lines: I) -> u32
    where I : Iterator<Item = String> {
    let mut vec : Vec<([usize; VALUE_SIZE], u32)> = Vec::new();
    for line in lines {
        let hb = parse_hand_and_bid(&line);
        vec.push(hb);
    }
    vec.sort_by(|(l,_),(r,_)| compare_hands(l,r));
    let mut sum : u32 = 0;
    for (i,(_,b)) in vec.iter().enumerate() {
        sum += ((i as u32) + 1) * b;
    }
    sum
}

fn parse_hand_and_bid(line: &str) -> ([usize; VALUE_SIZE], u32) {
    let hand = parse_hand(&line[..HAND_SIZE]);
    let &bid = &line[HAND_SIZE+1..].parse::<u32>().unwrap();
    (hand, bid)
}

fn compare_hands(l : &[usize; VALUE_SIZE], r : &[usize; VALUE_SIZE]) -> Ordering {
    for i in 0..l.len() {
        match l[i].cmp(&r[i]) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            _ => {  }
        }
    }
    Ordering::Equal
}


fn parse_hand(hand : &str) -> [usize; VALUE_SIZE] {
    let mut counts = [0u32; CARD_ORDER.len()]; 
    let mut v = [0usize; VALUE_SIZE];
    let mut m1 = 0;
    let mut m2 = 0;
    for (j,c) in hand.chars().enumerate() {
        if let Some(i) = CARD_ORDER.find(c) {
            counts[i] += 1;
            let count = counts[i];
            if count > m1 { m1 = count }
            else if count > m2 { m2 = count }
            v[j + 1] = i;
        }
    }
    let class = match (m1,m2) {
        (5,0) => 6,
        (4,1) => 5, 
        (3,2) => 4, 
        (3,1) => 3,
        (2,2) => 2, 
        (2,1) => 1,
        (_,_) => 0
    };
    v[0] = class as usize;
    v
}
