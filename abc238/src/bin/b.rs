use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut a_accumed = vec![0];
    for i in 0..n {
        a_accumed.push((a_accumed[i] + a[i]) % 360);
    }
    a_accumed.push(360);
    a_accumed.sort();
    let mut degrees = Vec::new();
    for i in 0..(a_accumed.len() - 1) {
        degrees.push(a_accumed[i + 1] - a_accumed[i]);
    }

    println!("{}", degrees.iter().max().unwrap());
}
