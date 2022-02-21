use proconio::input;

fn main() {
    input! {
        s: [String; 3],
        t: proconio::marker::Chars
    }
    for c in t {
        let i = c as usize - '0' as usize - 1;
        print!("{}", s[i]);
    }
    println!("");
}
