use proconio::input;

fn main() {
    input! {
        p: [usize; 26]
    }
    for i in 0..26 {
        print!("{}", ('a' as u8 + p[i] as u8 - 1) as char);
    }
    println!("");
}
