// https://leetcode.com/problems/find-peak-element/
pub struct Solution;

impl Solution {
    pub fn find_peak_element(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut l = 0;
        let mut r = n;

        while r - l > 1 {
            let m = (l + r) / 2;
            if a[m - 1] < a[m] {
                l = m;
            } else {
                r = m;
            }
        }

        l as i32
    }
}

fn main() {
    let nums = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    println!("{}", Solution::find_peak_element(nums));
}
