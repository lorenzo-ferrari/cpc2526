// https://codeforces.com/contest/545/problem/C
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(n: usize, x: Vec<i32>, h: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut l: i32 = i32::MIN;

        for i in 0..n {
            if x[i] - h[i] > l {
                ans += 1;
            } else if i + 1 == n || x[i] + h[i] < x[i + 1] {
                ans += 1;
                l = x[i] + h[i];
            }
            l = l.max(x[i]);
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let mut x = vec![0i32; n];
    let mut h = vec![0i32; n];
    for i in 0..n {
        x[i] = ints.next().unwrap();
        h[i] = ints.next().unwrap();
    }

    println!("{}", Solution::solve(n, x, h));
}
