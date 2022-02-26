use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut pastas = HashMap::new();
    for a_i in a {
        *pastas.entry(a_i).or_insert(0) += 1;
    }
    for b_i in b {
        *pastas.entry(b_i).or_insert(0) -= 1;
    }
    println!(
        "{}",
        if pastas.iter().all(|k| *k.1 >= 0) {
            "Yes"
        } else {
            "No"
        }
    );
}
