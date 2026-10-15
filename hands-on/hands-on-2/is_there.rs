// Hands-On 2; Problem #1
use std::io::{self, Read};

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            res.push(a[i]);
            i += 1;
        } else {
            res.push(b[j]);
            j += 1;
        }
    }
    res.extend_from_slice(&a[i..]);
    res.extend_from_slice(&b[j..]);
    res
}

pub struct Solution;
struct MergeSortTree {
    n: usize,
    t: Vec<Vec<i32>>,
}

impl MergeSortTree {
    pub fn new(a: Vec<i32>) -> Self {
        let n = a.len();
        let mut t = vec![vec![-1; 0usize]; 2 * n];
        for i in 0..n {
            t[i + n].push(a[i]);
        }
        for i in (1..n).rev() {
            t[i] = merge(&t[2 * i], &t[2 * i + 1]);
        }

        Self { n, t }
    }
    
    pub fn is_there(&self, mut l: usize, mut r: usize, k: i32) -> bool {
        l += self.n;
        r += self.n;
        while l < r {
            if l % 2 == 1 {
                if self.t[l].binary_search(&k).is_ok() {
                    return true;
                }
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                if self.t[r].binary_search(&k).is_ok() {
                    return true;
                }
            }
            l /= 2;
            r /= 2;
        }

        false
    }
}

impl Solution {
    pub fn solve(segs: Vec<(usize, usize)>, queries: Vec<(usize, usize, i32)>) -> Vec<bool> {
        let n = segs.len();
        let mut frq = vec![0i32; n + 1];
        for (l, r) in segs {
            frq[l] += 1;
            frq[r + 1] -= 1;
        }
        for i in 1..=n {
            frq[i] += frq[i - 1];
        }

        let st = MergeSortTree::new(frq);

        let mut ans = Vec::<bool>::with_capacity(queries.len());
        for (l, r, k) in queries {
            ans.push(st.is_there(l, r + 1, k));
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let m = ints.next().unwrap() as usize;
    let mut segs = vec![(0usize, 0usize); n];
    let mut queries = vec![(0usize, 0usize, 0i32); m];
    for i in 0..n {
        let l = ints.next().unwrap() as usize;
        let r = ints.next().unwrap() as usize;
        segs[i] = (l, r);
    }
    for i in 0..m {
        let l = ints.next().unwrap() as usize;
        let r = ints.next().unwrap() as usize;
        let k = ints.next().unwrap();
        queries[i] = (l, r, k);
    }

    for x in Solution::solve(segs, queries) {
        println!("{}", x as i32);
    }
}
