use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        a: [i64;n],
        b: [i64;n],
    }
    let mut ans1 = 0;
    for i in 0..n {
        if a[i] == b[i] {
            ans1 += 1;
        }
    }

    let a_set = a.iter().collect::<HashSet<_>>();
    let b_set = b.iter().collect::<HashSet<_>>();
    let ans2 = a_set.intersection(&b_set).collect_vec().len() - ans1;

    println!("{}\n{}", ans1, ans2);
}
