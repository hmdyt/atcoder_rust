use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    println!(
        "{}",
        if a * 3600 + b * 60 < c * 3600 + d * 60 + 1 {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
