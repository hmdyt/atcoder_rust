use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [isize; n],
        mut b: [isize; m],
    }
    a.sort();
    b.sort();

    let mut candies = Vec::new();

    for i in 0..m {
        let mut ok = 0;
        let mut ng = n - 1;
        while (ok as isize - ng as isize).abs() > 1 {
            let mid = (ok + ng) / 2;
            if a[mid] < b[i] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        let candi1 = (a[ok] - b[i]).abs() as usize;
        let candi2 = (a[ng] - b[i]).abs() as usize;
        candies.push(candi1);
        candies.push(candi2);
    }

    let ans = candies.iter().min().unwrap();
    println!("{}", ans);
}
