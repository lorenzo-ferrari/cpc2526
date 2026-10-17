pub struct Solution;

impl Solution {
    pub fn solve(n: usize, d: usize, attractions: Vec<Vec<u32>>) -> u32 {
        let mut dp = vec![0u32; d + 1];
        let mut prv = dp.clone();

        for i in 0..n {
            let mut acc = 0;
            for j in 0..d {
                acc += attractions[i][j];
                for k in (j + 1)..=d {
                    dp[k] = dp[k].max(prv[k - j - 1] + acc);
                }
            }
            prv = dp.clone();
        }

        dp.into_iter().max().unwrap_or(0)
    }
}
