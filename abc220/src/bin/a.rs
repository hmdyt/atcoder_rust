use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize)
    }

    let mut ans = -1;
    for i in a..=b {
        if i % c == 0 {
            ans = i as i32;
        }
    }
    println!("{}", ans);
}
