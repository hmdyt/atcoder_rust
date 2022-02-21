use proconio::input;

fn pow_usize(a: usize, b: usize) -> usize {
    let mut ret = 1;
    for _ in 0..b {
        ret *= a;
    }
    return ret;
}

fn k_to_10(n_k: usize, k: usize) -> usize {
    let mut ret = 0;
    for (i, n) in n_k
        .to_string()
        .chars()
        .rev()
        .map(|x| x as usize - '0' as usize)
        .enumerate()
    {
        ret += n * pow_usize(k, i);
    }
    return ret;
}
fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize
    }
    println!("{}", k_to_10(a, k) * k_to_10(b, k));
}
