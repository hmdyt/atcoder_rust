use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Chars,
    }
    let ans = match s[n - 1] {
        'o' => "Yes",
        'x' => "No",
        _ => "Err",
    };
    println!("{}", ans);
}
