use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans;
    if 0 < a && b == 0 {
        ans = "Gold";
    } else if a == 0 && 0 < b {
        ans = "Silver";
    } else {
        ans = "Alloy";
    }

    println!("{}", ans);
}
