use proconio::input;

fn main() {
    input! {
        mut v: i64,
        a: i64,
        b: i64,
        c: i64,
    }
    let who = ["F", "M", "T"];
    for i in 0..1000000 {
        match i % 3 {
            0 => {
                v -= a;
            }
            1 => {
                v -= b;
            }
            2 => {
                v -= c;
            }
            _ => {}
        }
        if v < 0 {
            println!("{}", who[(i % 3) as usize]);
            break;
        }
    }
}
