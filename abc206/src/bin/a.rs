use proconio::input;

fn main() {
    input!{
        n: f32,
    }
    let ans: i32 = (n * 1.08) as i32;
    let ans_char: &str;
    if ans == 206 {
        ans_char = "so-so";
    } else if ans < 206 {
        ans_char = "Yay!";
    } else {
        ans_char = ":(";
    }
    println!("{}", ans_char);
}
