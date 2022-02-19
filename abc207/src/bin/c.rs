use proconio::input;

fn convert_section(tlr: &Vec<i64>) -> [f64; 2]{
    let ret: [f64; 2];
    let l = tlr[1] as f64;
    let r = tlr[2] as f64;
    match tlr[0] {
        1 => ret = [l, r],
        2 => ret = [l, r - 0.1],
        3 => ret = [l + 0.1, r],
        4 => ret = [l + 0.1, r - 0.1],
        _ => ret = [0., 0.]
    }
    return ret;
}

fn is_cover(lr1: [f64; 2], lr2: [f64; 2]) -> bool{
    let ret;
    if (lr1[0] <= lr2[0] && lr2[0] <= lr1[1]) || (lr1[0] <= lr2[1] && lr2[1] <= lr1[1]){
        ret = true;
    } else if (lr2[0] <= lr1[0] && lr1[0] <= lr2[1]) || (lr2[0] <= lr1[1] && lr1[1] <= lr2[1]){
        ret = true;
    } else {
        ret = false;
    }
    return ret;
}

fn main() {
    input! {
        n: usize,
        tlr_s: [[i64; 3]; n]
    }
    let mut ans: i64 = 0;

    for i in 0..n {
        for j in i+1..n {
            ans += (is_cover(
                convert_section(&tlr_s[i]),
                convert_section(&tlr_s[j])
            )) as i64;
        }
    }
    println!("{}", ans);
}
