use libp1::Solution;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let m = ints.next().unwrap() as usize;
    let mut segs = vec![(0usize, 0usize); n];
    let mut queries = vec![(0usize, 0usize, 0i32); m];
    for i in 0..n {
        let l = ints.next().unwrap() as usize;
        let r = ints.next().unwrap() as usize;
        segs[i] = (l, r);
    }
    for i in 0..m {
        let l = ints.next().unwrap() as usize;
        let r = ints.next().unwrap() as usize;
        let k = ints.next().unwrap();
        queries[i] = (l, r, k);
    }

    for x in Solution::solve(segs, queries) {
        println!("{}", x as i32);
    }
}
