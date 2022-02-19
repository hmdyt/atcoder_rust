use proconio::input;
fn main() {
    input! {
        s: proconio::marker::Chars,
    }
    let mod_num: usize = (10e+9 as usize) + 7;
    let chokudai: Vec<char> = "chokudai".chars().collect();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; chokudai.len()]; s.len()];

    for i in 0..s.len() {
        for j in 0..chokudai.len() {
            if j == 0 {
                dp[i][j] = 1;
            } else {
                if i == 0 {
                    dp[i][j] = 0;
                } else if s[i] != chokudai[j] {
                    dp[i][j] = dp[i - 1][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
                }
            }
            dp[i][j] %= mod_num;
        }
    }

    let ans = dp.last().unwrap().last().unwrap();
    println!("{:?}", ans);
}
