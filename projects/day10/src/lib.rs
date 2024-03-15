const VALID_LEFT_CHARS : &str = "-FL";
const VALID_RIGHT_CHARS : &str = "-J7";
const VALID_UP_CHARS : &str = "|F7";
const VALID_DOWN_CHARS : &str = "|JL";


pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    let mut i = 0;
    let mut j = 0;
    let mut v: Vec<Vec<char>> = Vec::new();
    for (index,line) in lines.enumerate() {
        v.push(line.chars().collect());
        match &line.find('S') {
            Some(s) => { i = *s; j = index; },
            None => { }
        }
    }
    let mut di = 0isize;
    let mut dj = 0isize;
    let mut len = 0;
    let m = v[0].len() - 1;
    let n = v.len() - 1;
    
    // before we start looping work out a direction to travel in
    if i > 0 && VALID_LEFT_CHARS.contains(v[j][i - 1]) {
        di = -1;
    }
    else if j > 0 && VALID_UP_CHARS.contains(v[j - 1][i]) {
        dj = -1;
    }
    else if i < m - 1 && VALID_RIGHT_CHARS.contains(v[j][i + 1]) {
        di = 1;
    }
    else if j < n - 1 && VALID_DOWN_CHARS.contains(v[j + 1][i]) {
        dj = 1;
    }
    else {
        panic!("Could not find starting direction from 'S' at ({i},{j})");
    }
    i = ((i as isize) + di) as usize;
    j = ((j as isize) + dj) as usize;

    loop {
        let c = v[j][i];
        len += 1;
        match c {
            'S' => { break; },
            '-' => { }, // di = +/- 1, dj = 0, do nothing
            '|' => { }, // di = 0, dj = +/- 1, do nothing
            'F' => (di,dj) = match(di,dj) {
                (-1,0) => (0,1),
                (0,-1) => (1,0),
                _ => panic!("Cannot enter F from N or W")
            },
            'J' => (di,dj) = match(di,dj) {
                (1,0) => (0,-1),
                (0,1) => (-1,0),
                _ => panic!("Cannot enter J from S or E")
            },
            'L' => (di,dj) = match(di,dj) {
                (-1,0) => (0,-1),
                (0,1) => (1,0),
                _ => panic!("Cannot enter L from S or W")
            },
            '7' => (di,dj) = match(di,dj) {
                (1,0) => (0,1),
                (0,-1) => (-1,0),
                _ => panic!("Cannot enter 7 from N or W")
            },
            _ => panic!("Entered unknown pipe {c}")
        }
        i = ((i as isize) + di) as usize;
        j = ((j as isize) + dj) as usize;
    }
    len / 2
}


