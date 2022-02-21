use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize
    }
    let a_sum: usize = a.iter().sum();
    let n_a_iteration = x / a_sum;
    let current_num = a_sum * n_a_iteration;

    let mut nokori = 0;
    let mut nokori_wa = 0;
    for i in 0..n {
        nokori_wa += a[i];
        if nokori_wa > (x - current_num) {
            nokori = i;
            break;
        }
    }
    println!("{}", n * n_a_iteration + nokori + 1);
}
