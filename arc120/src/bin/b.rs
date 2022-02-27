use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [proconio::marker::Chars; h]
    }
    const MOD: usize = 998244353;
    let mut diag_tiles: Vec<HashSet<char>> = vec![HashSet::new(); h + w];
    for i in 0..h {
        for j in 0..w {
            diag_tiles[i + j].insert(s[i][j]);
        }
    }

    let mut ans = 1;
    for tiles in diag_tiles {
        if tiles.contains(&'R') && tiles.contains(&'B') {
            ans *= 0
        } else if tiles.contains(&'.') && tiles.len() == 1 {
            ans *= 2;
        }
        ans %= MOD;
    }
    println!("{}", ans);
}
