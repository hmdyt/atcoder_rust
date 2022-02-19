use proconio::input;

fn main() {
    input!{
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let diff = c*d -b;
    if 0 >= diff { 
        println!("-1");
    } else {
        println!("{}", (a+diff-1)/diff);
    }
}
