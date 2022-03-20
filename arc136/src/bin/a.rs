use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Chars,
    }
    let mut s_b = Vec::new();
    for i in 0..n {
        if s[i] == 'A' {
            s_b.push('B');
            s_b.push('B');
        } else {
            s_b.push(s[i]);
        }
    }

    let mut s_a = Vec::new();
    let mut j = 0;
    while j < s_b.len() {
        if s_b[j] == 'B' && j + 1 < s_b.len() {
            if s_b[j + 1] == 'B' {
                s_a.push('A');
                j += 1;
            } else {
                s_a.push(s_b[j]);
            }
        } else {
            s_a.push(s_b[j]);
        }
        j += 1;
    }
    println!("{}", s_a.iter().join(""));
}
