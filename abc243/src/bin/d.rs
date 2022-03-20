use itertools::Itertools;
use proconio::input;

fn pow_usize(x: usize, n: usize) -> usize {
    let mut ret = 1;
    for _ in 0..n {
        ret *= x;
    }
    return ret;
}

fn convert_10_to_2(n_input: usize) -> Vec<String> {
    let mut n = n_input;
    let mut ret = Vec::new();

    for i in (0..64).rev() {
        let p = pow_usize(2, i);
        let digit = n / p;
        n %= p;
        ret.push(digit.to_string());
    }

    return ret;
}

fn main() {
    input! {
        n: usize,
        x: usize,
        s: proconio::marker::Chars
    }
    let mut ans = convert_10_to_2(x);
    for i in 0..n {
        match s[i] {
            'U' => {
                ans.pop();
            }
            'R' => {
                ans.push("1".to_string());
            }
            'L' => {
                ans.push("0".to_string());
            }
            _ => {}
        }
    }
    println!(
        "{:?}",
        usize::from_str_radix(ans.iter().join("").as_str(), 2).unwrap()
    );
}
