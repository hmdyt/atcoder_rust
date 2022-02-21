use proconio::input;

fn main() {
    input! {
        n: usize,
        name_split: [(String, String); n]
    }
    let names = name_split
        .iter()
        .map(|x| format!("{} {}", x.0, x.1))
        .collect::<Vec<_>>();

    let mut ans = "No";
    for i in 0..n {
        for j in 0..n {
            if names[i] == names[j] && i != j {
                ans = "Yes";
            }
        }
    }

    println!("{}", ans);
}
