// https://cses.fi/problemset/task/3403/
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let m = b.len();

        let mut dp: Vec<Vec<usize>> = vec![vec![0usize; m + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=m {
                if a[i - 1] == b[j - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                }
                dp[i][j] = dp[i][j].max(dp[i - 1][j]);
                dp[i][j] = dp[i][j].max(dp[i][j - 1]);
            }
        }

        let mut ans:Vec<i32> = Vec::with_capacity(dp[n][m]);
        let mut i = n;
        let mut j = m;
        while i > 0 && j > 0 {
            if a[i - 1] == b[j - 1] {
                ans.push(a[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i][j] == dp[i - 1][j] {
                i -= 1;
            } else {
                j -= 1;
            }
        }

        ans.reverse();
        ans
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let m = ints.next().unwrap() as usize;
    let mut a = vec![0i32; n];
    let mut b = vec![0i32; m];
    for i in 0..n {
        a[i] = ints.next().unwrap();
    }
    for i in 0..m {
        b[i] = ints.next().unwrap();
    }

    let ans = Solution::solve(a, b);
    println!("{}", ans.len());
    for x in ans {
        print!("{} ", x);
    }
    println!();
}
