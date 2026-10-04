// https://cses.fi/problemset/task/1629/
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(n: usize, a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut intervals: Vec<(i32, i32)> = a.iter().copied().zip(b.iter().copied()).collect();
        intervals.sort_unstable();

        let mut suf_min: Vec<i32> = vec![i32::MAX; n];
        suf_min[n - 1] = intervals[n - 1].1;
        for i in (0..(n-1)).rev() {
            suf_min[i] = suf_min[i + 1].min(intervals[i].1);
        }

        let mut l = i32::MIN;
        let mut ans = 0;
        for i in 0..n {
            if intervals[i].0 >= l {
                ans += 1;
                l = suf_min[i];
            }
        }

        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let mut a = vec![0i32; n];
    let mut b = vec![0i32; n];
    for i in 0..n {
        a[i] = ints.next().unwrap();
        b[i] = ints.next().unwrap();
    }

    println!("{}", Solution::solve(n, a, b));
}
