use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut s: proconio::marker::Chars
    }
    s.sort();
    println!("{}", s.iter().join(""));
}
