use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut points: [(usize, usize); n],
        s: proconio::marker::Chars
    }
    let mut points_map: HashMap<(usize, char), Vec<usize>> = HashMap::new();
    let mut y_set = HashSet::new();
    for i in 0..n {
        let (x, y) = points[i];
        y_set.insert(y);
        points_map.entry((y, 'R')).or_insert(vec![]);
        points_map.entry((y, 'L')).or_insert(vec![]);
        points_map.entry((y, s[i])).or_insert(vec![]).push(x);
    }
    let mut ans = "No";
    for y in y_set {
        if points_map[&(y, 'R')].len() == 0 || points_map[&(y, 'L')].len() == 0 {
            continue;
        }
        let candi_r = *points_map[&(y, 'R')].iter().min().unwrap();
        let candi_l = *points_map[&(y, 'L')].iter().max().unwrap();
        if candi_r < candi_l {
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
