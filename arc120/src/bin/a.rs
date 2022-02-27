use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut a_accum = vec![a[0]];
    let mut a_accum_accum = vec![a_accum[0]];
    for i in 0..n - 1 {
        a_accum.push(a_accum[i] + a[i + 1]);
    }
    for i in 0..n - 1 {
        a_accum_accum.push(a_accum_accum[i] + a_accum[i + 1]);
    }

    let mut a_max = a[0];
    println!("{}", a[0] * 2);
    for i in 1..n {
        a_max = std::cmp::max(a_max, a[i]);
        println!("{}", a_accum_accum[i] + (i + 1) * a_max);
    }
}
