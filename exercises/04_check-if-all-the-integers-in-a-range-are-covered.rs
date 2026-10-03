// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered
pub struct Solution;

impl Solution {
    pub fn is_covered(mut ranges: Vec<Vec<i32>>, mut l: i32, r: i32) -> bool {
        ranges.sort_unstable();
        for range in &ranges {
            if l < range[0] {
                return false;
            }
            l = l.max(range[1] + 1);
            if l > r {
                return true;
            }
        }

        false
    }
}

fn main() {
    let ranges = vec![vec![1,2], vec![3,4], vec![5,6]];
    let l = 2;
    let r = 5;
    println!("{}", Solution::is_covered(ranges, l, r));
}
