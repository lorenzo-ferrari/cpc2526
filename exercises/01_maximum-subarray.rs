// https://leetcode.com/problems/maximum-subarray/
pub struct Solution;

impl Solution {
    pub fn max_sub_array(a: Vec<i32>) -> i32 {
        let mut prf = 0;
        let mut min_prf = 0;
        let mut ans = a[0];

        for &x in &a[0..] {
            prf = prf + x;
            ans = ans.max(prf - min_prf);
            min_prf = min_prf.min(prf);
        }

        ans
    }
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    println!("{}", Solution::max_sub_array(nums));
}
