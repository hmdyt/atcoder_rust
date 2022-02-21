use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n]
    }

    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i] - 1] = i + 1;
    }

    println!(
        "{}",
        q.iter()
            .map(|q_i| { format!("{}", q_i) })
            .collect::<Vec<_>>()
            .join(" ")
    );
}
