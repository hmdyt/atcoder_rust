use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let mut dp = vec![vec![false; 100000]; n];
    input! {a0: usize, b0: usize}
    dp[0][a0] = true;
    dp[0][b0] = true;

    for i in 1..n {
        input! {a: usize, b: usize}
        for j in 0..10000 {
            if dp[i - 1][j] {
                dp[i][j + a] = true;
                dp[i][j + b] = true;
            }
        }
    }

    println!("{}", if dp[dp.len() - 1][x] { "Yes" } else { "No" });
}
