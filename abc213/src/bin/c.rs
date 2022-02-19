use proconio::input;
use std::collections::{HashMap, HashSet};

fn zip_cord(in_vec: Vec<usize>) -> HashMap<usize, usize> {
    let mut ret = HashMap::new();
    for i in 0..in_vec.len() {
        let k = in_vec[i];
        let v = i;
        ret.insert(k, v);
    }
    return ret;
}

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        points: [(usize, usize); n],
    }
    let mut x_set: HashSet<usize> = HashSet::new();
    let mut y_set: HashSet<usize> = HashSet::new();
    for i in 0..n {
        x_set.insert(points[i].0);
        y_set.insert(points[i].1);
    }
    let mut x_sorted = x_set.into_iter().collect::<Vec<usize>>();
    let mut y_sorted = y_set.into_iter().collect::<Vec<usize>>();
    x_sorted.sort();
    y_sorted.sort();
    let x_ziped = zip_cord(x_sorted);
    let y_ziped = zip_cord(y_sorted);
    for i in 0..n {
        let (x, y) = points[i];
        println!("{} {}", x_ziped[&x] + 1, y_ziped[&y] + 1);
    }
}
