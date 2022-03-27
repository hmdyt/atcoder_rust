use proconio::input;

fn _is_capable(a: usize, b: usize, c: usize, d: usize) -> bool {
    a <= c && b <= d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; m],
        d: [usize; m],
    }
    let mut choco = (0..n).map(|i| (a[i], b[i])).collect::<Vec<_>>();
    let mut hako = (0..m).map(|i| (c[i], d[i])).collect::<Vec<_>>();
    choco.sort_by_key(|x| x.0);
    choco.sort_by_key(|x| x.1);
    hako.sort_by_key(|x| x.0);
    hako.sort_by_key(|x| x.1);
}
