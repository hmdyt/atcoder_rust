use proconio::input;

fn main() {
    input! {
        s: String
    }
    let v = s.split('.').collect::<Vec<_>>();
    let x: usize = v[0].parse().unwrap();
    let y: usize = v[1].parse().unwrap();

    println!(
        "{}{}",
        x,
        match y {
            0..=2 => "-",
            3..=6 => "",
            7..=9 => "+",
            _ => "-1",
        }
    )
}
