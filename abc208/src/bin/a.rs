use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let ans: &str;
    if a <= b && b <= 6*a { ans = "Yes" }
    else { ans = "No" }
    println!("{}", ans)
}
