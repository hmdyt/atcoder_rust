use proconio::input;

fn make_points(x1: i64, y1: i64) -> [(i64, i64); 8] {
    let pp: [i64; 2] = [-1, 1];
    let dd: [i64; 2] = [1, 2];
    let mut ret = [(x1, y1); 8];
    let mut i = 0;
    for p_x in pp.iter() {
        for p_y in pp.iter() {
            for x in dd.iter() {
                for y in dd.iter() {
                    if x == y {
                        continue;
                    }
                    let dx = p_x * x;
                    let dy = p_y * y;
                    ret[i].0 += dx;
                    ret[i].1 += dy;
                    i += 1;
                }
            }
        }
    }
    return ret;
}

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }
    let mut ans = false;
    let points1 = make_points(x1, y1);
    let points2 = make_points(x2, y2);
    for p1 in points1.iter() {
        for p2 in points2.iter() {
            ans |= p1.0 == p2.0 && p1.1 == p2.1;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
