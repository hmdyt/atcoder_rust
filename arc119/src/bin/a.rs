use proconio::input;

fn pow_i128(a: i128, n: i128) -> i128 {
    let mut ret = 1;
    for _ in 0..n {
        ret *= a;
    }
    return ret;
}

fn main() {
    input! {
        n: i128
    }
    let mut ans = Vec::new();
    for b in 0..60 {
        let b2 = pow_i128(2, b);
        let a = n / b2;
        let c = n - a * b2;
        if c >= 0 {
            ans.push(a + b + c);
        }
    }
    println!("{}", ans.iter().min().unwrap());
}
