use proconio::input;
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};

fn proccess_que(a: &mut BTreeMap<i64, i64>) {
    input! {c: usize}
    match c {
        1 => {
            input! {x: i64}
            *a.entry(x).or_insert(0) += 1;
        }
        2 => {
            input! {x: i64, mut k: i64}
            let mut a_lower_x = a.range((Unbounded, Included(x)));
            //println!("{:?}", a_lower_x.next_back());
            loop {
                match a_lower_x.next_back() {
                    Some(res) => {
                        k -= res.1;
                        if k <= 0 {
                            println!("{}", res.0);
                            break;
                        }
                    }
                    None => {
                        println!("-1");
                        break;
                    }
                }
            }
        }
        3 => {
            input! {x: i64, mut k: i64}
            let mut a_upper_x = a.range((Included(x), Unbounded));
            //println!("{:?}", a_upper_x);
            loop {
                match a_upper_x.next() {
                    Some(res) => {
                        k -= res.1;
                        if k <= 0 {
                            println!("{}", res.0);
                            break;
                        }
                    }
                    None => {
                        println!("-1");
                        break;
                    }
                }
            }
        }
        _ => {}
    }
}

fn main() {
    input! {
        q: usize
    }
    let mut a = BTreeMap::new();
    for _ in 0..q {
        proccess_que(&mut a);
    }
}
