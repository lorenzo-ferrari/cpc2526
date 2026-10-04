// https://leetcode.com/problems/continuous-subarray-sum
use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut set = BTreeSet::new();

        let mut prv = 0;
        let mut prf = 0;
        for x in nums {
            prf = (prf + x) % k;
            if set.contains(&prf) {
                return true;
            }
            set.insert(prv);
            prv = prf;
        }
        false
    }
}

fn main() {
    let nums = vec![23, 2, 4, 6, 6];
    let k = 7;
    println!("{}", Solution::check_subarray_sum(nums, k));
}
