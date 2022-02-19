use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Chars,
    }

    let mut warui_index: usize = 0;
    for i in 0..n {
        if s[i] == '1' {
            warui_index = i;
            break;
        }
    }

    let ans = if warui_index % 2 == 0 {
        "Takahashi"
    } else {
        "Aoki"
    };
    println!("{}", &ans);
}
