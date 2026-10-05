// https://codeforces.com/problemset/problem/652/D
use std::io::{self, Read};

pub struct Solution;
struct SegTree {
    n: usize,
    t: Vec<i32>,
}
struct SegTreeRangePoint {
    st: SegTree,
}

impl SegTree {
    pub fn new(n: usize) -> Self {
        Self { n, t: vec![0i32; 2 * n] }
    }
    
    pub fn increase(&mut self, mut p: usize, x: i32) {
        p += self.n;
        while p > 0 {
            self.t[p] += x;
            p /= 2;
        }
    }

    pub fn sum_range(&self, mut l: usize, mut r: usize) -> i32 {
        let mut ans = 0;
        l += self.n;
        r += self.n;
        while l < r {
            if l % 2 == 1 {
                ans += self.t[l];
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                ans += self.t[r];
            }
            l /= 2;
            r /= 2;
        }
        ans
    }
}

impl SegTreeRangePoint {
    pub fn new(n: usize) -> Self {
        Self {
            st: SegTree::new(n + 1)
        }
    }

    pub fn increase(&mut self, l: usize, r: usize, x: i32) {
        self.st.increase(l, x);
        self.st.increase(r, -x);
    }

    pub fn query(&self, p: usize) -> i32 {
        self.st.sum_range(0, p + 1)
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
            *x = zip.binary_search(x).unwrap() as i32;
        }
        for x in &mut r {
            *x = zip.binary_search(x).unwrap() as i32;
        }

        let mut segments = Vec::<(i32, i32, usize)>::with_capacity(n);
        for i in 0..n {
            segments.push((l[i], r[i], i));
        }

        segments.sort_unstable_by_key(|seg| seg.1 - seg.0);

        let mut ans = vec![0i32; n];
        let mut st1 = SegTree::new(2 * n + 1);
        let mut st2 = SegTreeRangePoint::new(2 * n + 1);

        for (li, ri, i) in segments {
            ans[i] = (st1.sum_range(li as usize, ri as usize) - st2.query(li as usize) - st2.query(ri as usize)) / 2 as i32;
            st1.increase(li as usize, 1);
            st1.increase(ri as usize, 1);
            st2.increase(li as usize, ri as usize, 1);
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
