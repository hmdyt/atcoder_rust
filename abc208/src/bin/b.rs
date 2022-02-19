use proconio::input;

fn facto(n: i64) -> i64 {
    if n == 1 {return 1}
    return n * facto(n-1)
}

fn main() {
    input!{mut p: i64}
    let mut ans = 0;
    for i in (1..11).rev() {
        let coin = facto(i);
        while p >= coin {
            p -= coin;
            ans += 1;
        }
    }
    println!("{}", ans);
}
