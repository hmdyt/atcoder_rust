use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        s: proconio::marker::Chars,
        k: usize
    }

    let mut s_set = std::collections::HashSet::new();
    for p in s.iter().permutations(s.len()) {
        let s_vec_i: String = p.iter().map(|x| *x).collect();
        s_set.insert(s_vec_i);
    }
    let mut s_sorted = s_set.iter().collect::<Vec<_>>();
    s_sorted.sort();

    println!("{}", s_sorted[k - 1]);
}
