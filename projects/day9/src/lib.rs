use utils::*;

pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    next_in_sequence::<I>(lines, 0, |m, n| n - m + 1 as i64)
}

pub fn part_two<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    next_in_sequence::<I>(lines, 1, |_,_| 0)
}


pub fn next_in_sequence<I>(lines: I, s: i64, t: fn(i64, i64) -> i64) -> i64
    where I : Iterator<Item = String> {
    let mut sum = 0;
    for line in lines {
        let a = custom_parse_to_vec::<i64>(&line, " ", |x| x.parse::<i64>().expect("error"));
        let m = get_sequence_depth(&a);
        let n = a.len() as i64;
        let o = t(m,n);
        let v = (-1i64).pow(s as u32) * binomial_sum(&a, s, s + m - 2, o, m - 1);
        sum += v;
    }
    sum
}

// calculate number of diff sequences needed to find 0 difference
fn get_sequence_depth(a: &Vec<i64>) -> i64 {
    let n = (a.len() - 1) as i64;
    let mut b_p = 1;
    for m in 1..n {
        let b = binomial_sum(&a, 0, m, n - m - 1, m);
        if b == 0 && b_p == 0 {
            return m
        }
    }
    n
}

// given a sequence a_i
// calculate the sum from k = k_start to k_end 
// of (-1)^k (n choose k) a_{k - k_offset}
fn binomial_sum(a: &Vec<i64>, k_start: i64, k_end: i64, i_start: i64, n: i64) -> i64 {
    let mut sum = 0;
    for k in k_start..=k_end {
        let i = i_start + k - k_start;
        let s = (-1i64).pow(k as u32);
        let b = choose(n, k);
        let a_i = a[i as usize];
        let s_i = s * b * a_i;
        sum += s_i;
    }
    sum
}

fn choose(n: i64, k: i64) -> i64 {
    let mut v = 1;
    let k = if k < n - k { k } else { n - k };
    for p in n-k+1..=n { v *= p; }
    for p in 2..=k { v /= p; }
    v
}
