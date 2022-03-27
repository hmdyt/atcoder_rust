use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let mut dp: Vec<Vec<bool>> = vec![vec![true, true]; n];
    for i in 1..n {
        dp[i][0] = dp[i - 1][0] && (num::abs(a[i] - a[i - 1]) <= k)
            || dp[i - 1][1] && (num::abs(a[i] - b[i - 1]) <= k);
        dp[i][1] = dp[i - 1][0] && (num::abs(b[i] - a[i - 1]) <= k)
            || dp[i - 1][1] && (num::abs(b[i] - b[i - 1]) <= k);
    }
    println!(
        "{}",
        if dp[dp.len() - 1][0] || dp[dp.len() - 1][1] {
            "Yes"
        } else {
            "No"
        }
    );
}
