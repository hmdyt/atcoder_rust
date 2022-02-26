use std::usize;

use proconio::input;

fn is_ok(o_i: usize, o_j: usize, s: &Vec<Vec<char>>) -> bool {
    let mut ret = false;
    // horizon
    for i in 0..6 {
        let mut cnt = 0;
        for j in 0..6 {
            if s[o_i + i][o_j + j] == '#' {
                cnt += 1;
            }
        }
        ret |= cnt > 3;
    }
    // vertical
    for i in 0..6 {
        let mut cnt = 0;
        for j in 0..6 {
            if s[o_i + j][o_j + i] == '#' {
                cnt += 1;
            }
        }
        ret |= cnt > 3;
    }
    // diag
    {
        let mut cnt = 0;
        for i in 0..6 {
            if s[o_i + i][o_j + i] == '#' {
                cnt += 1;
            }
        }
        ret |= cnt > 3;
    }
    {
        let mut cnt = 0;
        for i in 0..6 {
            if s[o_i + 5 - i][o_j + i] == '#' {
                cnt += 1;
            }
        }
        ret |= cnt > 3;
    }
    return ret;
}

fn main() {
    input! {
        n: usize,
        s: [proconio::marker::Chars; n]
    }
    let mut ans = false;
    for i in 0..(n - 5) {
        for j in 0..(n - 5) {
            ans |= is_ok(i, j, &s);
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
