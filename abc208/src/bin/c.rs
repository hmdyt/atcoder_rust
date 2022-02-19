use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut people = Vec::<(i64, i64)>::new();
    for i in 0..n { people.push((i as i64, a[i] as i64)); }
    people.sort_by(|a, b| (-b.1).cmp(&(-a.1)));

    let sho = (k / n) as i64;
    let amari = k % n;
    let mut ans = vec![sho; n];

    for i in 0..amari {
        let target_index: usize = people[i].0 as usize;
        ans[target_index] += 1;
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
