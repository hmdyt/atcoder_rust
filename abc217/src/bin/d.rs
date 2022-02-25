use proconio::input;
use std::collections::BTreeSet;

fn neighbors(tree: &BTreeSet<usize>, val: usize) -> (Option<&usize>, Option<&usize>) {
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Excluded(val)));
    let mut after = tree.range((Excluded(val), Unbounded));

    (before.next_back(), after.next())
}

fn main() {
    input! {
        l: usize,
        q: usize
    }
    let mut plank = BTreeSet::new();
    plank.insert(0);
    plank.insert(l);
    for _ in 0..q {
        input! {c: usize, x: usize}
        match c {
            1 => {
                plank.insert(x);
            }
            2 => {
                let (prev, next) = neighbors(&plank, x);
                println!("{}", next.unwrap() - prev.unwrap());
            }
            _ => {}
        }
    }
}
