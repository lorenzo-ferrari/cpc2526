// https://codeforces.com/problemset/problem/596/B
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(b: Vec<i64>) -> i64 {
        let mut ans: i64 = 0;
        let mut cur: i64 = 0;
        for x in b {
            ans += cur.abs_diff(x) as i64;
            cur = x;
        }
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());

    let n = ints.next().unwrap() as usize;
    let b: Vec<i64> = ints.take(n).collect();

    println!("{}", Solution::solve(b));
}
