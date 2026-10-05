// https://codeforces.com/problemset/problem/652/D
use std::io::{self, Read};

pub struct Solution;
struct FenwickTree {
    ft: Vec<i32>,
}
struct FenwickTreeRangePoint {
    bit: FenwickTree,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self { ft: vec![0i32; n + 2] }
    }
    
    pub fn increase(&mut self, mut p: usize, x: i32) {
        while p < self.ft.len() {
            self.ft[p] += x;
            p += p & (!p + 1);
        }
    }

    pub fn sum(&self, mut p: usize) -> i32 {
        let mut ans = 0;
        while p > 0 {
            ans += self.ft[p];
            p = p & (p - 1);
        }
        ans
    }

    pub fn sum_range(&self, l: usize, r: usize) -> i32 {
        self.sum(r - 1) - if l != 0 { self.sum(l - 1) } else { 0 }
    }
}

impl FenwickTreeRangePoint {
    pub fn new(n: usize) -> Self {
        Self {
            bit: FenwickTree::new(n)
        }
    }

    pub fn increase(&mut self, l: usize, r: usize, x: i32) {
        self.bit.increase(l, x);
        self.bit.increase(r, -x);
    }

    pub fn query(&self, p: usize) -> i32 {
        self.bit.sum_range(0, p + 1)
    }
}

impl Solution {
    pub fn solve(n: usize, mut l: Vec<i32>, mut r: Vec<i32>) -> Vec<i32> {
        // compress
        let mut zip = Vec::with_capacity(l.len() + r.len());
        zip.extend_from_slice(&l);
        zip.extend_from_slice(&r);
        zip.sort_unstable();
        zip.dedup();

        for x in &mut l {
            *x = (zip.binary_search(x).unwrap() + 1) as i32;
        }
        for x in &mut r {
            *x = (zip.binary_search(x).unwrap() + 1) as i32;
        }

        let mut segments = Vec::<(i32, i32, usize)>::with_capacity(n);
        for i in 0..n {
            segments.push((l[i], r[i], i));
        }

        segments.sort_unstable_by_key(|seg| seg.1 - seg.0);

        let mut ans = vec![0i32; n];
        let mut ft1 = FenwickTree::new(2 * n + 1);
        let mut ft2 = FenwickTreeRangePoint::new(2 * n + 1);

        for (li, ri, i) in segments {
            ans[i] = (ft1.sum_range(li as usize, ri as usize) - ft2.query(li as usize) - ft2.query(ri as usize)) / 2 as i32;
            ft1.increase(li as usize, 1);
            ft1.increase(ri as usize, 1);
            ft2.increase(li as usize, ri as usize, 1);
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let mut l = vec![0i32; n];
    let mut r = vec![0i32; n];
    for i in 0..n {
        l[i] = ints.next().unwrap();
        r[i] = ints.next().unwrap();
    }

    for x in Solution::solve(n, l.clone(), r.clone()) {
        println!("{}", x);
    }
}
