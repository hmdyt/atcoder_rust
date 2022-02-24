use std::usize;

use proconio::input;

fn solve_one(a: usize, s: usize) -> &'static str {
    if (s as i128) - 2 * (a as i128) >= 0 {
        if a & (s - 2 * a) == 0 {
            "Yes"
        } else {
            "No"
        }
    } else {
        "No"
    }
}

fn main() {
    input! {
        t: usize,
        a_s: [(usize, usize); t]
    }
    for i in 0..t {
        println!("{}", solve_one(a_s[i].0, a_s[i].1));
    }
}
