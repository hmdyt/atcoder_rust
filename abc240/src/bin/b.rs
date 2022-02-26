use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    println!("{}", a.into_iter().collect::<HashSet<usize>>().len());
}
