use proconio::input;

fn main() {
    input! { mut n: u64 }
    let mut ans = Vec::new();
    while n > 0 {
        match n % 2 == 0 {
            true => {
                n /= 2;
                ans.push('B')
            }
            false => {
                n -= 1;
                ans.push('A');
            }
        }
    }
    println!("{}", ans.iter().rev().collect::<String>());
}
