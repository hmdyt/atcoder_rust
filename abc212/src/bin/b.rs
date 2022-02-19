use proconio::input;
use std::collections::HashSet;

fn get_next(n: usize) -> usize {
    let ret: usize;
    if n == 9 {
        ret = 0;
    } else {
        ret = n + 1;
    }
    return ret;
}

fn main() {
    input! {
        xxxx: proconio::marker::Chars
    }
    let mut pass = Vec::new();
    for i in xxxx {
        pass.push(i as usize - '0' as usize);
    }

    let ans;
    let mut flags = vec![false, false, false];
    for i in 0..flags.len() {
        flags[i] = pass[i + 1] == get_next(pass[i]);
    }

    if pass.clone().into_iter().collect::<HashSet<usize>>().len() == 1 {
        ans = "Weak";
    } else if flags[0] && flags[1] && flags[2] {
        ans = "Weak";
    } else {
        ans = "Strong";
    }

    println!("{}", ans);
}
