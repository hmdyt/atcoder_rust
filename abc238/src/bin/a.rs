use libm::log2;
use proconio::input;

fn main() {
    input! {
        n: f64
    }
    println!("{}", if n > 2. * log2(n) { "Yes" } else { "No" });
}
