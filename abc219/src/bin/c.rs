use itertools::Itertools;

fn main() {
    proconio::input! {
        x: proconio::marker::Chars,
        n: usize,
        s: [String; n]
    }
    let mut char_to_int = std::collections::HashMap::new();
    for i in 0..26 {
        char_to_int.insert(x[i], i);
    }

    let int_vec = s
        .iter()
        .map(|s_i| s_i.chars().map(|c| char_to_int[&c]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut s_int_vec = Vec::new();
    for i in 0..n {
        s_int_vec.push((&s[i], &int_vec[i]));
    }
    s_int_vec.sort_by_key(|x| x.1);

    println!("{}", s_int_vec.iter().map(|x| (*x).0).join("\n"));
}
