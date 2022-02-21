use proconio::input;
use std::collections::HashMap;

fn pow_u128(a: u128, b: u128) -> u128 {
    let mut ret = 1;
    for _ in 0..b {
        ret *= a;
    }
    return ret;
}

fn alphabet_to_base10(s: &String, d: &HashMap<char, u128>) -> u128 {
    let mut ret = 0;
    let s_rev: Vec<char> = s.chars().rev().collect();
    for i in 0..s_rev.len() {
        ret += d[&s_rev[i]] * pow_u128(26, i as u128);
    }
    return ret;
}

fn main() {
    input! {
        x: proconio::marker::Chars,
        n: usize,
        s: [String; n]
    }
    let mut b26_b10: HashMap<char, u128> = HashMap::new();
    for i in 0..x.len() {
        b26_b10.insert(x[i], i as u128);
    }

    let mut s_base10 = Vec::new();
    for s_i in &s {
        s_base10.push((s_i, alphabet_to_base10(s_i, &b26_b10)));
    }
    s_base10.sort_by_key(|x| (*x).1);

    for i in &s_base10 {
        println!("{}", i.0);
    }
    //println!("{:?}", &s_base10);
}

#[test]
fn test_pow_u128() {
    assert_eq!(pow_u128(26, 0), 1);
    assert_eq!(pow_u128(26, 1), 26);
    assert_eq!(pow_u128(26, 6), 308915776);
}

#[test]
fn test_alphabet_to_base10() {
    let x = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let mut b26_b10 = HashMap::new();
    for i in 0..x.len() {
        b26_b10.insert(x[i], i as u128);
    }
    println!("{:?}", b26_b10);
    assert_eq!(alphabet_to_base10(&"a".to_string(), &b26_b10), 0);
    assert_eq!(alphabet_to_base10(&"aa".to_string(), &b26_b10), 0);
    assert_eq!(alphabet_to_base10(&"ab".to_string(), &b26_b10), 1);
    assert_eq!(alphabet_to_base10(&"ba".to_string(), &b26_b10), 26);
    assert_eq!(alphabet_to_base10(&"za".to_string(), &b26_b10), 25 * 26);
    assert_eq!(alphabet_to_base10(&"ccc".to_string(), &b26_b10), 1406);
    assert_eq!(alphabet_to_base10(&"cccc".to_string(), &b26_b10), 36558);
    assert_eq!(alphabet_to_base10(&"ccccc".to_string(), &b26_b10), 950510);
    assert_eq!(alphabet_to_base10(&"zyx".to_string(), &b26_b10), 17547);
}
