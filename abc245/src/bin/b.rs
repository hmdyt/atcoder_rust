use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let a_set = a.iter().collect::<HashSet<_>>();
    for i in 0..2001 {
        if !a_set.contains(&(i as usize)) {
            println!("{}", i);
            return;
        }
    }
}
