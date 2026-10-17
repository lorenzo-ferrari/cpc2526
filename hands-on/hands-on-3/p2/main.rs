use libp2::Solution;
use std::io::{self, Read};

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
