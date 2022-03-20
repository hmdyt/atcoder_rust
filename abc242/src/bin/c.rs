use num::abs;
use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![1; 9]; n];
    for i in 1..n {
        for x_i in 1..10 {
            let j: usize = x_i - 1;
            let n_add = match x_i {
                1 => 2,
                9 => 2,
                _ => 3,
            };
            dp[i][j] = dp[i - 1][j] * n_add;
            dp[i][j] %= 998244353;
        }
    }

    println!("{:?}", dp[dp.len() - 1].iter().sum::<usize>());
    println!("{:?}", dp[dp.len() - 1]);
}
