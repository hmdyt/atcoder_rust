use proconio::input;

fn main() {
    input! {
        x: i128
    }
    let mut ans = x / 10;
    if x < 0 && x % 10 != 0 {
        ans -= 1;
    }
    println!("{}", ans);
}
