// Hands-On 3; Problem #1
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(n: usize, d: usize, attractions: Vec<Vec<u32>>) -> u32 {
        let mut dp = vec![0u32; d + 1];
        let mut prv = dp.clone();

        for i in 0..n {
            let mut acc = 0;
            for j in 0..d {
                acc += attractions[i][j];
                for k in (j+1)..=d {
                    dp[k] = dp[k].max(prv[k - j - 1] + acc);
                }
            }
            prv = dp.clone();
        }

        dp.into_iter().max().unwrap_or(0)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let d = ints.next().unwrap() as usize;
    let mut attractions: Vec<Vec<u32>> = vec![vec![0u32; d]; n];
    for i in 0..n {
        for j in 0..d {
            attractions[i][j] = ints.next().unwrap() as u32;
        }
    }

    println!("{}", Solution::solve(n, d, attractions));
}
