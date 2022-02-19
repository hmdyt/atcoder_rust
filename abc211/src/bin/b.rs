use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: [String; 4]
    }
    let s_set: HashSet<String> = s.into_iter().collect();
    let ans;
    if s_set.len() == 4 {
        ans = "Yes";
    } else {
        ans = "No";
    }
    println!("{}", ans);
}
