// https://leetcode.com/problems/jump-game-ii/

pub struct Solution;

struct SegTree {
    n: usize,
    t: Vec<i32>,
}

impl SegTree {
    pub fn new(n: usize) -> Self {
        Self { n, t: vec![1_000_000_000; 2 * n] }
    }

    pub fn setmin(&mut self, mut p: usize, x: i32) {
        p += self.n;
        while p > 0 {
            self.t[p] = self.t[p].min(x);
            p /= 2;
        }
    }

    pub fn min_range(&self, mut l: usize, mut r: usize) -> i32 {
        let mut ans = i32::MAX;
        l += self.n;
        r += self.n;
        while l < r {
            if l % 2 == 1 {
                ans = ans.min(self.t[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                ans = ans.min(self.t[r]);
            }
            l /= 2;
            r /= 2;
        }
        ans
    }
}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0i32; nums.len() + 1];
        let mut st = SegTree::new(nums.len() + 1);
        dp[nums.len() - 1] = 0;
        st.setmin(nums.len() - 1, 0);
        for i in (0..(nums.len() - 1)).rev() {
            let l = i;
            let mut r = i + nums[i] as usize;
            r = r.min(nums.len());
            dp[i] = 1 + st.min_range(l, r + 1);
            st.setmin(i, dp[i]);
        }

        dp[0]
    }
}

fn main() {
    let nums = vec![2, 3, 0, 1, 4];
    println!("{}", Solution::jump(nums));
}
