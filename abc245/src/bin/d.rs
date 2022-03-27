use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i128; n + 1],
        c: [i128; n+m+1],
    }

    let v_a = a.into_iter().rev().collect::<Vec<_>>();
    let mut v_b = Vec::new();
    let mut v_c = c.into_iter().rev().collect::<Vec<_>>();
    for i in 0..(m + 1) {
        let multip = v_c[i] / v_a[0];
        v_b.push(multip);
        for j in 0..(n + 1) {
            v_c[i + j] -= v_a[j] * multip;
        }
    }
    println!("{}", v_b.iter().rev().join(" "));
}
