use proconio::input;

fn main() {
    input! {
        n: i128
    }

    let mut candi = 1;
    let mut k = 0;

    while candi <= n {
        candi *= 2;
        k += 1;
    }

    println!("{}", k - 1);
}
