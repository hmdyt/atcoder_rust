use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    }

    let ans: usize;
    if n > a {
        ans = a * x + (n - a) * y;
    } else {
        ans = n * x;
    }
    println!("{}", ans);
}
