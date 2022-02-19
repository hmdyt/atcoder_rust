use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    let ans = b - a + 1;
    if ans < 0 {
        println!("0");
    } else {
        println!("{}", ans);
    }
}
