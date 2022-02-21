use proconio::input;

fn main() {
    input! {
        n: usize
    }

    println!(
        "{}",
        match n {
            1..=125 => 4,
            126..=211 => 6,
            212..=214 => 8,
            _ => -1,
        }
    );
}
