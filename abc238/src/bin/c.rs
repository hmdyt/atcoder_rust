use proconio::input;

fn pow_i128(a: i128, b: i128) -> i128 {
    // return a**b
    let mut ret = 1;
    for _ in 0..b {
        ret *= a;
    }
    return ret;
}

fn main() {
    const MOD: i128 = 998244353;
    input! {
        n: i128
    }
    let n_keta = n.to_string().len() as i128;
    let mut ans = 0;

    for i in 0..(n_keta - 1) {
        if i == 0 {
            ans += 45;
        } else {
            let n_max = pow_i128(10, i + 1) - pow_i128(10, i);
            ans += n_max * (n_max + 1) / 2;
            ans %= MOD;
        }
    }
    let n_max = n - pow_i128(10, n_keta - 1) + 1;
    ans += n_max * (n_max + 1) / 2;
    ans %= MOD;
    println!("{}", ans);
}
