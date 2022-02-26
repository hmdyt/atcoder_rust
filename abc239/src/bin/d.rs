use proconio::input;

fn is_prime(n: usize) -> bool {
    if n < 2 {
        false
    } else if n == 2 {
        true
    } else if n % 2 == 0 {
        false
    } else {
        let mut i = 3;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }
        true
    }
}

fn main() {
    input! {a: usize, b:usize, c: usize, d: usize}
    for i in a..(b + 1) {
        let mut resulets = Vec::new();
        for j in c..(d + 1) {
            resulets.push(!is_prime(i + j));
        }
        if resulets.iter().all(|x| *x == true) {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
