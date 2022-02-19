use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }

    let mut candies: HashMap<usize, usize> = HashMap::new();
    for i in 0..k {
        match candies.get(&c[i]) {
            Some(candies_c_i) => {
                let tmp = candies_c_i + 1;
                candies.insert(c[i], tmp);
            }
            None => {
                candies.insert(c[i], 1);
            }
        }
    }

    let mut ans = candies.keys().len();
    for i in k..n {
        let c_delete = c[i - k];
        let c_insert = c[i];
        match candies.get(&c_insert) {
            Some(candies_c_i) => {
                let tmp = candies_c_i + 1;
                candies.insert(c_insert, tmp);
            }
            None => {
                candies.insert(c_insert, 1);
            }
        }
        match candies.get(&c_delete) {
            Some(candies_c_i) => {
                let tmp = candies_c_i - 1;
                if tmp == 0 {
                    candies.remove(&c_delete);
                } else {
                    candies.insert(c_delete, tmp);
                }
            }
            None => {
                return ();
            }
        }
        ans = std::cmp::max(ans, candies.keys().len());
    }

    println!("{}", ans);
}
