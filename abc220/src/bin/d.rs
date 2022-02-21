use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    const MOD: usize = 998244353;
    let mut dp = vec![vec![0 as usize; 10]; n];
    dp[0][a[0]] = 1;
    for i in 1..n {
        for j in 0..10 {
            let target1 = (j + a[i]) % 10;
            let target2 = (j * a[i]) % 10;
            dp[i][target1] += dp[i - 1][j];
            dp[i][target2] += dp[i - 1][j];
            dp[i][target1] %= MOD;
            dp[i][target2] %= MOD;
        }
    }
    println!(
        "{}",
        dp[dp.len() - 1].iter().map(|x| format!("{}", x)).join("\n")
    );
}
