use proconio::input;
use std::collections::{HashMap, HashSet};

fn compress(in_vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut use_cor_x_set = HashSet::new();
    let mut use_cor_y_set = HashSet::new();
    for i in 0..in_vec.len() {
        for j in 0..in_vec.len() {
            if in_vec[i][j] == '#' {
                use_cor_x_set.insert(i);
                use_cor_y_set.insert(j);
            }
        }
    }
    let mut use_cor_x_list = (*use_cor_x_set.iter().min().unwrap()
        ..=*use_cor_x_set.iter().max().unwrap())
        .collect::<Vec<_>>();
    let mut use_cor_y_list = (*use_cor_y_set.iter().min().unwrap()
        ..*use_cor_y_set.iter().max().unwrap())
        .collect::<Vec<_>>();
    use_cor_x_list.sort();
    use_cor_y_list.sort();
    let mut zip_map_x = HashMap::new();
    let mut zip_map_y = HashMap::new();
    for i in 0..use_cor_x_list.len() {
        zip_map_x.insert(i, use_cor_x_list[i]);
    }
    for i in 0..use_cor_y_list.len() {
        zip_map_y.insert(i, use_cor_y_list[i]);
    }

    let mut ret = vec![vec!['x'; use_cor_y_list.len()]; use_cor_x_list.len()];
    for i in 0..ret.len() {
        for j in 0..ret[i].len() {
            ret[i][j] = in_vec[zip_map_x[&i]][zip_map_y[&j]];
        }
    }
    return ret;
}

fn rotate90(in_vec: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = in_vec.len();
    let mut ret = vec![vec!['x'; n]; n];
    for i in 0..n {
        for j in 0..n {
            ret[i][j] = in_vec[j][n - 1 - i];
        }
    }
    return ret;
}

fn is_same_shape(v1: &Vec<Vec<char>>, v2: &Vec<Vec<char>>) -> bool {
    let v1_compd = compress(&v1);
    let v2_compd = compress(&v2);
    let v1_shape = (v1_compd.len(), v1_compd[0].len());
    let v2_shape = (v2_compd.len(), v2_compd[0].len());
    if v1_shape == v2_shape {
        let mut flags = Vec::new();
        for i in 0..v1_shape.0 {
            for j in 0..v1_shape.1 {
                flags.push(v1_compd[i][j] == v2_compd[i][j]);
            }
        }
        return flags.iter().all(|x| *x == true);
    } else {
        return false;
    }
}

fn main() {
    input! {
        n: usize,
        s: [proconio::marker::Chars; n],
        t: [proconio::marker::Chars; n],
    }

    let mut candies = vec![s];
    for i in 0..3 {
        candies.push(rotate90(&candies[i]));
    }

    let ans = (0..4).map(|i| is_same_shape(&t, &candies[i])).any(|x| x);
    println!("{}", if ans { "Yes" } else { "No" });
}
