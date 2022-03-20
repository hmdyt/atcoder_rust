use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }
    let mut a_counter = HashMap::new();
    let mut b_counter = HashMap::new();
    for i in 0..n {
        *a_counter.entry(a[i]).or_insert(0) += 1;
        *b_counter.entry(b[i]).or_insert(0) += 1;
    }
    if a_counter != b_counter {
        println!("No");
        return;
    }
    for (_, v) in a_counter {
        if v > 1 {
            println!("Yes");
            return;
        }
    }
    for (_, v) in b_counter {
        if v > 1 {
            println!("Yes");
            return;
        }
    }

    for i in 0..(n - 2) {
        match b.iter().position(|x| *x == a[i]) {
            Some(e) => {
                b.remove(e);
            }
            None => {}
        };
    }

    println!(
        "{}",
        if a[a.len() - 1] == b[b.len() - 1] {
            "Yes"
        } else {
            "No"
        }
    );
}
