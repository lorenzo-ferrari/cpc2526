use libp2::Solution;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let m = ints.next().unwrap() as usize;
    let mut a = vec![0i32; n];
    let mut is_query = vec![false; m];
    let mut l = vec![0usize; m];
    let mut r = vec![0usize; m];
    let mut t = vec![0i32; m];
    for i in 0..n {
        a[i] = ints.next().unwrap();
    }
    for i in 0..m {
        is_query[i] = ints.next().unwrap() != 0;
        l[i] = (ints.next().unwrap() - 1) as usize;
        r[i] = (ints.next().unwrap() - 1) as usize;
        if !is_query[i] {
            t[i] = ints.next().unwrap();
        }
    }

    for x in Solution::solve(m, a, is_query, l, r, t) {
        println!("{}", x);
    }
}
