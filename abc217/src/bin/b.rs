use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: [String; 3]
    }
    let input_set = s.iter().collect::<HashSet<_>>();
    let contest_vec = vec![
        "ABC".to_string().clone(),
        "ARC".to_string().clone(),
        "AGC".to_string().clone(),
        "AHC".to_string().clone(),
    ];
    let contest_set = contest_vec.iter().collect::<HashSet<_>>();

    let ans = contest_set.difference(&input_set).collect::<Vec<_>>();
    println!("{}", ans[0]);
}
