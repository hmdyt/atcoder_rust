use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut p: [(i64, i64); n]
    }
    let points = (0..n)
        .zip(p)
        .map(|x| (x.0, x.1 .0, x.1 .1))
        .collect::<Vec<_>>();
    let mut candidate_points = HashSet::new();
    let points_x_sorted = (&points)
        .into_iter()
        .sorted_by(|x, y| x.1.cmp(&y.1))
        .collect::<Vec<_>>();
    let points_y_sorted = (&points)
        .into_iter()
        .sorted_by(|x, y| x.2.cmp(&y.2))
        .collect::<Vec<_>>();

    for points_sorted in &[points_x_sorted, points_y_sorted] {
        candidate_points.insert(*points_sorted[0]);
        candidate_points.insert(*points_sorted[1]);
        candidate_points.insert(*points_sorted[points_sorted.len() - 1]);
        candidate_points.insert(*points_sorted[points_sorted.len() - 2]);
    }
    let mut ans_candis = Vec::new();
    for candidate_point in &candidate_points {
        for point in &points {
            ans_candis.push((
                std::cmp::min(candidate_point.0, point.0),
                std::cmp::max(candidate_point.0, point.0),
                std::cmp::max(
                    (candidate_point.1 - point.1).abs(),
                    (candidate_point.2 - point.2).abs(),
                ),
            ));
        }
    }

    let ans = ans_candis
        .into_iter()
        .unique()
        .sorted_by(|x, y| x.2.cmp(&y.2))
        .collect_vec();
    println!("{:?}", ans[ans.len() - 2].2);
}
