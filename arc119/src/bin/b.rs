use proconio::input;

fn main() {
    input! {
        n: usize,
        s: proconio::marker::Chars,
        t: proconio::marker::Chars,
    }
    if s.iter().filter(|&x| *x == '1').count() == t.iter().filter(|&x| *x == '1').count() {
        let mut s_i = Vec::new();
        let mut t_i = Vec::new();
        for i in 0..n {
            if s[i] == '0' {
                s_i.push(i);
            }
            if t[i] == '0' {
                t_i.push(i);
            }
        }
        let ans = s_i.into_iter().zip(t_i).filter(|&(x, y)| x != y).count();
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}
