// https://www.spoj.com/problems/UPDATEIT/
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(n: usize, u: usize, l: Vec<i32>, r: Vec<i32>, v: Vec<i32>, q: usize, p: Vec<i32>) {
        let mut prf = vec![0i32; n + 1];
        for i in 0..u {
            prf[l[i] as usize] += v[i];
            prf[(r[i] + 1) as usize] -= v[i];
        }
        for i in 1..(n+1) {
            prf[i] += prf[i - 1];
        }
        for i in 0..q {
            println!("{}", prf[p[i] as usize]);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let t = ints.next().unwrap();
    for _ in 0..t {
        let n = ints.next().unwrap() as usize;
        let u = ints.next().unwrap() as usize;
        let mut l = vec![0i32; u];
        let mut r = vec![0i32; u];
        let mut v = vec![0i32; u];
        for j in 0..u {
            l[j] = ints.next().unwrap();
            r[j] = ints.next().unwrap();
            v[j] = ints.next().unwrap();
        }
        let q = ints.next().unwrap() as usize;
        let mut p = vec![0i32; q];
        for j in 0..q {
            p[j] = ints.next().unwrap();
        }
        Solution::solve(n, u, l, r, v, q, p);
    }
}
