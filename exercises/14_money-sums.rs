// https://cses.fi/problemset/task/1745/
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(n: usize, a: Vec<usize>) -> Vec<usize> {
        const K: usize = 1000;
        let m: usize = n * K;
        
        let mut dp = vec![false; m + 1];
        dp[0] = true;
        for x in a {
            for i in (x..=m).rev() {
                dp[i] |= dp[i - x];
            }
        }
        let mut ans = vec![];
        for x in 1..=m {
            if dp[x] {
                ans.push(x);
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
    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = ints.next().unwrap() as usize;
    }
    let ans = Solution::solve(n, a);

    println!("{}", ans.len());
    for x in ans {
        print!("{} ", x);
    }
    println!();
}
