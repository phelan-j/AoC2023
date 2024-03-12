use utils::*;

pub fn part_one<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    next_in_sequence::<I>(lines)
}

pub fn next_in_sequence<I>(lines: I) -> i64
    where I : Iterator<Item = String> {
    let mut sum = 0;
    for line in lines {
        let v = custom_parse_to_vec::<i64>(&line, " ", |x| x.parse::<i64>().expect("error"));
        let mut b_p = 1;
        for i in 1..v.len() {
            let b = diff_sum(&v, v.len() - i - 1, i);
            if b == 0 && b_p == 0 {
                let n = (-1i64).pow(i as u32) * diff_sum(&v, v.len() - i + 1, i - 1);
                sum += n;
                break;
            }
            b_p = b;
        }
        if b_p != 0 {
            let n = diff_sum(&v, 0, v.len());
            sum += n;
        }
    }
    sum
}

fn diff_sum(vec: &Vec<i64>, o: usize, n: usize) -> i64 {
    let mut sum = 0;
    let k_max = if o + n < vec.len() - 1 { n } else { vec.len() - 1 - o };
    for k in 0..=k_max {
        let s = (-1i64).pow(k as u32);
        let b = choose(n as i64, k as i64);
        let v = vec[o + k];
        sum += s * b * v;
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
