use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut a_map = HashMap::<i64, i64>::new();

    for i in &a {
        let count = a_map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut ans = (n * (n - 1) / 2) as i64;

    for a_map_i in a_map.iter() {
        ans -= a_map_i.1 * (a_map_i.1 - 1) / 2;
    }

    println!("{}", ans);
}
