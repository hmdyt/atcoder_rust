use libm::sqrt;
use proconio::input;

fn main() {
    input! {
        h: f64
    }
    println!("{}", sqrt(h * (12800000. + h)));
}
