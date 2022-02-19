use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64,
        mut a: [i64; n as usize]
    }

    for i in 0..(n as usize) {
        if i % 2 == 1 {
            a[i] -= 1;
        }
    }

    let sum_a: i64 = a.iter().sum();
    let ans: &str;

    if sum_a > x {
        ans = "No";
    } else {
        ans = "Yes";
    }
    println!("{}", ans);
}
