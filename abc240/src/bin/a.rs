use proconio::input;

fn main() {
    input! {
        a: i8,
        b: i8
    }
    println!(
        "{}",
        if (a - b).abs() == 1 || (a - b).abs() == 9 {
            "Yes"
        } else {
            "No"
        }
    );
}
