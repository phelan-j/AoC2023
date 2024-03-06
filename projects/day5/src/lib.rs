use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_to_vec(line: &str) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new(); 
    let mut val = None;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if let Some(d) = c.to_digit(10) {
            let d = d as u64;
            val = match val {
                Some(v) => Some(10 * v + d),
                None => Some(d)
            }
        }
        else if let Some(v) = val { 
            vec.push(v);
            val = None;
        }
    }
    if let Some(v) = val { vec.push(v); }
    vec
}

pub fn part_one<I>(mut flattened: I)
    where I : Iterator<Item = String> {
    // load in first line to read seeds
    let line = flattened.next().unwrap();
    // read in vector of seeds - mutable as we will update
    let seeds = parse_to_vec(&line);

    let mut map_full: Vec<(u64,u64,u64)> = Vec::new();
    let mut map_curr: Vec<(u64,u64,u64)> = Vec::new();
    for line in flattened {
        let v = parse_to_vec(&line);
        if v.len() > 2 {
            map_curr.push((v[1],v[0],v[2]));
        }
        else if map_curr.len() > 0 {
            map_full = compose(&map_full, &map_curr);
            map_curr = Vec::new();
        }
    }
    if map_curr.len() > 0 {
        map_full = compose(&map_full, &map_curr);
    }
    map_full.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    let mut m_min : Option<u64> = None;
    for s in seeds {
        let m = apply_map(s, &map_full);
        m_min = Some(match m_min {
            Some(min) => if m < min { m } else { min },
            None => m
        });
    }
    if let Some(m_min) = m_min { println!("{m_min}"); }
}

fn apply_map(s: u64, map: &Vec<(u64,u64,u64)>) -> u64 {
    let mut map = map.clone();
    map.sort_by(|(x,_,_),(y,_,_)| x.cmp(y));
    let i = match map.binary_search_by(|(x,_,_)| x.cmp(&s)) {
        Ok(i) => i,
        Err(i) => if i > 0 { i - 1 } else { i }
    };
    let (ss, ds, l) = map[i];
    let se = ss + l;
    if ss <= s && s < se { ds + (s - ss) } else { s }
}

fn compose(f: &Vec<(u64,u64,u64)>, g: &Vec<(u64,u64,u64)>) -> Vec<(u64,u64,u64)> {
    let mut h : Vec<(u64,u64,u64)> = Vec::new();
    let mut f = f.clone();
    let mut g = g.clone();
    f.sort_by(|(_,x,_),(_,y,_)| { x.cmp(y) });
    g.sort_by(|(x,_,_),(y,_,_)| { x.cmp(y) });
    
    let mut f_iter = f.iter();
    let mut g_iter = g.iter();
    let mut f_v = f_iter.next();
    let mut g_v = g_iter.next();
    let mut r = 0;
    loop {
        if let Some(&(f_ss,f_ds,f_l)) = f_v {
        // f[  )
            let f_se = f_ss + f_l;
            let f_de = f_ds + f_l;
            
            let f_l = if r <= f_ds { f_l } else { f_de - r };
            
            let f_ss = f_se - f_l;
            let f_ds = f_de - f_l;
            
            if f_l > 0 {
                if let Some(&(g_ss,g_ds,g_l)) = g_v {
                    // g[  )
                    let g_se = g_ss + g_l;
                    let g_de = g_ds + g_l;
                    
                    let g_l = if r <= g_ss { g_l } else { g_se - r };
    
                    let g_ss = g_se - g_l;
                    let g_ds = g_de - g_l;
                    
                    if g_l > 0 {
                        if f_ds <= g_ss {
                            // f[ 
                            //     g[ 
                            if f_de <= g_ss { 
                                // f[  )
                                //        g[  )
                                h.push((f_ss,f_ds,f_l));
                                f_v = f_iter.next();
                            }
                            else {
                                // f[     )
                                //    g[  
                                let s = g_ss - f_ds;
                                if s > 0 {
                                    h.push((f_ss,f_ds,s));
                                }
                                if f_de <= g_se {
                                    // f[     )
                                    //    g[    )
                                    // split map
                                    h.push((f_ss + s,g_ds,f_l-s));
                                    r = f_de;
                                    f_v = f_iter.next();
                                } 
                                else {
                                    // f[        )
                                    //    g[  )
                                    h.push((f_ss + s,g_ds,g_l));
                                    r = g_se;
                                    g_v = g_iter.next();
                                }
                            }
                        }
                        else {
                            //     f[
                            // g[ 
                            if g_se <= f_ds {
                                //        f[
                                // g[  )
                                h.push((g_ss,g_ds,g_l));
                                g_v = g_iter.next();
                            }
                            else {
                                //    f[
                                // g[    )
                                let s = f_ds - g_ss;
                                if s > 0 {
                                    h.push((g_ss,g_ds,s));
                                }
                                if g_se < f_de {
                                    //    f[     )
                                    // g[     )
                                    // split map
                                    h.push((f_ss,g_ds+s,g_l-s));
                                    r = g_se;
                                    g_v = g_iter.next();
                                }
                                else {
                                    //    f[ )
                                    // g[      )
                                    h.push((f_ss,g_ds + s,f_l));
                                    r = f_de;
                                    f_v = f_iter.next();
                                }
                            }
                        }
                    }
                    else { g_v = g_iter.next(); }
                }
                else {
                    // f[  )
                    //
                    if f_l > 0 {
                        h.push((f_ss, f_ds, f_l));
                    }
                    f_v = f_iter.next();
                }
            }
            else { f_v = f_iter.next(); }
        }
        else if let Some(&(g_ss,g_ds,g_l)) = g_v {
            // 
            // g[  )
            let g_se = g_ss + g_l;
            let g_de = g_ds + g_l;
            
            let g_l = if r <= g_ss { g_l } else { g_se - r };
            
            let g_ss = g_se - g_l;
            let g_ds = g_de - g_l;
            
            if g_l > 0 {
                h.push((g_ss, g_ds, g_l));
            }
            
            g_v = g_iter.next();
        }
        else { break; }
    }
    h
}

#[cfg(test)]
mod test;
