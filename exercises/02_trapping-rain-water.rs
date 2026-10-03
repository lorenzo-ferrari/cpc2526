// https://leetcode.com/problems/trapping-rain-water/
pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut pmax = height.clone();
        let mut smax = height.clone();

        for i in 1..n {
            pmax[i] = pmax[i].max(pmax[i - 1]);
        }

        for i in (0..(n - 1)).rev() {
            smax[i] = smax[i].max(smax[i + 1]);
        }

        let mut sum = 0;
        for i in 0..n {
            sum = sum + pmax[i].min(smax[i]) - height[i];
        }

        sum
    }
}

fn main() {
    let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("{}", Solution::trap(height));
}
