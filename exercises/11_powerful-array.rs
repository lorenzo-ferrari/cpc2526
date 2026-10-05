// https://codeforces.com/contest/86/problem/D
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(n: usize, t: usize, a: Vec<i32>, l: Vec<usize>, r: Vec<usize>) -> Vec<i64> {
        const K: usize = 512;
        let mut intervals: Vec<(usize, usize, usize)> = Vec::with_capacity(t);
        for i in 0..t {
            intervals.push((l[i], r[i] + 1, i));
        }

        intervals.sort_unstable_by_key(|seg| n * (seg.0 / K) + seg.1 );

        let mut cur_l: usize = 0;
        let mut cur_r: usize = 0;
        let mut cur_val = 0i64;
        let mut frq = vec![0i64; 1_000_001];
        let mut ans = vec![0i64; t];

        let mut update = |idx: usize, delta: i64, cur_val: &mut i64| {
            let x = a[idx] as usize;
            *cur_val -= frq[x] * frq[x] * (x as i64);
            frq[x] += delta;
            *cur_val += frq[x] * frq[x] * (x as i64);
        };

        for (li, ri, i) in intervals {
            while cur_l < li {
                update(cur_l, -1, &mut cur_val);
                cur_l += 1;
            }
            while cur_l > li {
                cur_l -= 1;
                update(cur_l, 1, &mut cur_val);
            }
            while cur_r < ri {
                update(cur_r, 1, &mut cur_val);
                cur_r += 1;
            }
            while cur_r > ri {
                cur_r -= 1;
                update(cur_r, -1, &mut cur_val);
            }
            ans[i] = cur_val;
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let t = ints.next().unwrap() as usize;
    let mut a = vec![0i32; n];
    let mut l = vec![0usize; t];
    let mut r = vec![0usize; t];
    for i in 0..n {
        a[i] = ints.next().unwrap();
    }
    for i in 0..t {
        l[i] = ints.next().unwrap() as usize - 1;
        r[i] = ints.next().unwrap() as usize - 1;
    }

    for x in Solution::solve(n, t, a, l, r) {
        println!("{}", x);
    }
}
