use proconio::input;

fn main() {
    input! {
        n: usize,
        tmp: [usize; n],
    }
    let mut a = Vec::new();
    for i in 0..n {
        a.push((i, tmp[i]));
    }

    a.sort_by_key(|x| x.1);

    println!("{}", a[a.len() - 2].0 + 1);
}
