use proconio::input;

fn main() {
    input! {n: i32}
    let mut cash: i32 = 0;
    let mut i: i32 = 0;

    while cash < n {
        cash += i;
        i += 1;
    }

    println!("{}", i-1);
}
