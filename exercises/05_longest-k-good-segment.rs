// https://codeforces.com/contest/616/problem/D
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn longest_k_good_segment(a: Vec<i32>, k: i32) -> (usize, usize) {
        let mut frq = vec![0i32; 1_000_001];
        let mut cnt = 0;

        let mut r = 0;
        let mut l_opt = 0;
        let mut r_opt = 0;

        for l in 0..a.len() {
            while r < a.len() && (cnt < k || frq[a[r] as usize] != 0) {
                cnt += (frq[a[r] as usize] == 0) as i32;
                frq[a[r] as usize] += 1;
                r += 1;
            }

            if r - l > r_opt - l_opt {
                l_opt = l;
                r_opt = r;
            }

            frq[a[l] as usize] -= 1;
            cnt -= (frq[a[l] as usize] == 0) as i32;
        }

        (l_opt + 1, r_opt)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let k = ints.next().unwrap();
    let a: Vec<i32> = ints.take(n).collect();

    let (l, r) = Solution::longest_k_good_segment(a, k);
    println!("{l} {r}");
}
