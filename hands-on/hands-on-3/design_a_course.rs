// Hands-On 3; Problem #2
use std::io::{self, Read};

pub struct Solution;

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
}

impl Solution {
    pub fn solve(n: usize, mut pts: Vec<(i32, i32)>) -> u32 {
        if n == 0 {
            return 0;
        }

        // compress the second coordinate
        let mut y_vals: Vec<i32> = pts.iter().map(|&(_, b)| b).collect();
        y_vals.sort_unstable();
        y_vals.dedup();

        // sort by x coordinate and break ties based on (reverse) y coordinate
        pts.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

        let mut ft = FenwickTree::new(y_vals.len());
        let mut ans = 0;

        for (_, y) in pts {
            let y_rank = y_vals.binary_search(&y).unwrap() + 1;
            let best_prev = if y_rank > 1 { ft.query(y_rank - 1) } else { 0 };
            let cur_dp = best_prev + 1;
            ft.update(y_rank, cur_dp);
            ans = ans.max(cur_dp);
        }

        ans as u32
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

// vibe-coded stress test
#[cfg(test)]
mod tests {
    use super::*;

    fn naive_max_chain(mut pts: Vec<(i32, i32)>) -> u32 {
        let n = pts.len();
        if n == 0 {
            return 0;
        }
        pts.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));
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
    fn test_unconstrained_order() {
        let pts = vec![(4, 4), (1, 1), (3, 3), (2, 2)];
        assert_eq!(Solution::solve(pts.len(), pts), 4);
    }

    #[test]
    fn test_duplicates_and_ties() {
        let pts = vec![(1, 2), (1, 5), (1, 2), (2, 4), (3, 6)];
        assert_eq!(Solution::solve(pts.len(), pts.clone()), naive_max_chain(pts));
    }

    #[test]
    fn test_stress_random() {
        let mut state: u64 = 987654321;
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
                let expected = naive_max_chain(pts.clone());
                let actual = Solution::solve(n, pts);
                assert_eq!(actual, expected);
            }
        }
    }
}

