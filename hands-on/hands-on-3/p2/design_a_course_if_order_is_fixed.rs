// Hands-On 3; Problem #2 in case the relative order of the topics is fixed.
// without this assumption, the problem is much easier: see `design_a_course.rs`
use std::io::{self, Read};

pub struct Solution;

#[derive(Clone, Copy)]
pub struct Item {
    x: i32,
    y: usize,
    id: usize,
}

struct FenwickTree {
    n: usize,
    t: Vec<usize>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            t: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut p: usize, v: usize) {
        while p <= self.n {
            self.t[p] = self.t[p].max(v);
            p += p & p.wrapping_neg();
        }
    }

    fn query(&self, mut p: usize) -> usize {
        let mut ans = 0;
        while p > 0 {
            ans = ans.max(self.t[p]);
            p -= p & p.wrapping_neg();
        }
        ans
    }

    fn clear(&mut self, mut p: usize) {
        while p <= self.n {
            self.t[p] = 0;
            p += p & p.wrapping_neg();
        }
    }
}

impl Solution {
    pub fn solve(n: usize, pts: Vec<(i32, i32)>) -> u32 {
        if n == 0 {
            return 0;
        }

        // compression of the second coordinate
        let mut y_vals: Vec<i32> = pts.iter().map(|&(_, b)| b).collect();
        y_vals.sort_unstable();
        y_vals.dedup();

        let mut items: Vec<Item> = pts
            .iter()
            .enumerate()
            .map(|(id, &(x, y))| Item {
                x,
                y: y_vals.binary_search(&y).unwrap() + 1,
                id,
            })
            .collect();

        let mut dp = vec![1usize; n];
        let mut ft = FenwickTree::new(y_vals.len());

        Self::cdq(0, n, &mut items, &mut dp, &mut ft);

        dp.into_iter().max().unwrap_or(0) as u32
    }

    // CDQ Divide and Conquer
    fn cdq(
        l: usize,
        r: usize,
        items: &mut [Item],
        dp: &mut [usize],
        ft: &mut FenwickTree,
    ) {
        if r - l <= 1 {
            return;
        }

        let m = (l + r) / 2;

        Self::cdq(l, m, items, dp, ft);

        let mut left_part = items[l..m].to_vec();
        let mut right_part = items[m..r].to_vec();

        left_part.sort_unstable_by_key(|item| item.x);
        right_part.sort_unstable_by_key(|item| item.x);

        let mut j = 0;
        for right_item in &right_part {
            while j < left_part.len() && left_part[j].x < right_item.x {
                ft.update(left_part[j].y, dp[left_part[j].id]);
                j += 1;
            }

            if right_item.y > 1 {
                let from_left = ft.query(right_item.y - 1);
                if from_left > 0 {
                    dp[right_item.id] = dp[right_item.id].max(from_left + 1);
                }
            }
        }

        // make sure the Fenwick Tree is completely clear when entering cdq
        for item in &left_part[..j] {
            ft.clear(item.y);
        }

        Self::cdq(m, r, items, dp, ft);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let mut pts = vec![(0i32, 0i32); n];
    for i in 0..n {
        let x = ints.next().unwrap();
        let y = ints.next().unwrap();
        pts[i] = (x, y);
    }

    println!("{}", Solution::solve(n, pts));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_lis(pts: &[(i32, i32)]) -> u32 {
        let n = pts.len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![1u32; n];
        for i in 0..n {
            for j in 0..i {
                if pts[j].0 < pts[i].0 && pts[j].1 < pts[i].1 {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap_or(0)
    }

    #[test]
    fn test_empty_and_single() {
        assert_eq!(Solution::solve(0, vec![]), 0);
        assert_eq!(Solution::solve(1, vec![(5, 5)]), 1);
    }

    #[test]
    fn test_simple_cases() {
        // strictly increasing
        let pts1 = vec![(1, 1), (2, 2), (3, 3), (4, 4)];
        assert_eq!(Solution::solve(pts1.len(), pts1.clone()), naive_lis(&pts1));
        assert_eq!(Solution::solve(pts1.len(), pts1), 4);

        // strictly decreasing
        let pts2 = vec![(4, 4), (3, 3), (2, 2), (1, 1)];
        assert_eq!(Solution::solve(pts2.len(), pts2.clone()), naive_lis(&pts2));
        assert_eq!(Solution::solve(pts2.len(), pts2), 1);

        // duplicates
        let pts3 = vec![(2, 2), (2, 2), (2, 2)];
        assert_eq!(Solution::solve(pts3.len(), pts3.clone()), naive_lis(&pts3));
        assert_eq!(Solution::solve(pts3.len(), pts3), 1);
    }

    #[test]
    fn test_alternating_and_negatives() {
        let pts = vec![(-5, -5), (10, -2), (-3, -1), (0, 0), (5, 2), (2, 8), (6, 5), (10, 10)];
        assert_eq!(Solution::solve(pts.len(), pts.clone()), naive_lis(&pts));
    }

    #[test]
    fn test_stress_random() {
        let mut state: u64 = 123456789;
        let mut next_rand = || -> i32 {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((state >> 33) as i32) % 200 - 100
        };

        for &n in &[5, 10, 20, 50, 100, 200] {
            for _ in 0..10 {
                let mut pts = Vec::with_capacity(n);
                for _ in 0..n {
                    pts.push((next_rand(), next_rand()));
                }
                let expected = naive_lis(&pts);
                let actual = Solution::solve(n, pts);
                assert_eq!(actual, expected);
            }
        }
    }
}
