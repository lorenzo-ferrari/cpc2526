use libp1::Solution;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let d = ints.next().unwrap() as usize;
    let mut attractions: Vec<Vec<u32>> = vec![vec![0u32; d]; n];
    for i in 0..n {
        for j in 0..d {
            attractions[i][j] = ints.next().unwrap() as u32;
        }
    }

    println!("{}", Solution::solve(n, d, attractions));
}
