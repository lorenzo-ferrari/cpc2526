pub struct Solution;
struct SegTree {
    n: usize,
    t: Vec<i32>,
    lazy: Vec<i32>,
    lazy_set: Vec<bool>,
}

impl SegTree {
    pub fn new(a: Vec<i32>) -> Self {
        let mut seg_n = 1usize;
        while seg_n < a.len() {
            seg_n *= 2;
        }
        let mut seg_t = vec![i32::MIN; 2 * seg_n];
        for i in 0..a.len() {
            seg_t[i + seg_n] = a[i];
        }
        for i in (1..seg_n).rev() {
            seg_t[i] = seg_t[2 * i].max(seg_t[2 * i + 1]);
        }

        Self {
            n: seg_n,
            t: seg_t,
            lazy: vec![-1; 2 * seg_n],
            lazy_set: vec![false; 2 * seg_n],
        }
    }

    pub fn minch(&mut self, l: usize, r: usize, t: i32) {
        self.minch_rec(1usize, 0usize, self.n, l, r, t);
        return;
    }

    pub fn apply(&mut self, i: usize, t: i32) {
        self.t[i] = self.t[i].min(t);
        if !self.lazy_set[i] {
            self.lazy_set[i] = true;
            self.lazy[i] = t;
        } else {
            self.lazy[i] = self.lazy[i].min(t);
        }
    }

    pub fn prop(&mut self, i: usize) {
        if self.lazy_set[i] && i < self.n {
            // crash
            self.apply(2 * i, self.lazy[i]);
            self.apply(2 * i + 1, self.lazy[i]);
            self.lazy_set[i] = false;
        }
    }

    pub fn minch_rec(&mut self, i: usize, a: usize, b: usize, l: usize, r: usize, t: i32) {
        self.prop(i);
        if r <= a || b <= l {
            return;
        }
        if l <= a && b <= r {
            self.apply(i, t);
            return;
        }
        let m = (a + b) / 2;
        self.minch_rec(2 * i, a, m, l, r, t);
        self.minch_rec(2 * i + 1, m, b, l, r, t);
        self.t[i] = self.t[2 * i].max(self.t[2 * i + 1]);
    }

    pub fn max_range(&mut self, l: usize, r: usize) -> i32 {
        self.rec_max_range(1usize, 0usize, self.n, l, r)
    }

    pub fn rec_max_range(&mut self, i: usize, a: usize, b: usize, l: usize, r: usize) -> i32 {
        self.prop(i);
        if r <= a || b <= l {
            return i32::MIN;
        }
        if l <= a && b <= r {
            return self.t[i];
        }
        let m = (a + b) / 2;
        let min_l = self.rec_max_range(2 * i, a, m, l, r);
        let min_r = self.rec_max_range(2 * i + 1, m, b, l, r);

        min_r.max(min_l)
    }
}

impl Solution {
    pub fn solve(
        m: usize,
        a: Vec<i32>,
        is_query: Vec<bool>,
        l: Vec<usize>,
        r: Vec<usize>,
        t: Vec<i32>,
    ) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut st = SegTree::new(a);
        for i in 0..m {
            if is_query[i] {
                ans.push(st.max_range(l[i], r[i] + 1));
            } else {
                st.minch(l[i], r[i] + 1, t[i]);
            }
        }

        ans
    }
}
