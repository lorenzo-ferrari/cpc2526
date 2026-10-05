// https://cses.fi/problemset/task/1745/
use std::io::{self, Read};

pub struct Solution;

impl Solution {
    pub fn solve(a: Vec<usize>) -> usize {
        let mut stacks: Vec<usize> = Vec::new();

        for x in a {
            let idx = stacks.partition_point(|&top| top < x);

            if idx == stacks.len() {
                stacks.push(x);
            } else {
                stacks[idx] = x;
            }
        }
        
        stacks.len()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut ints = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let n = ints.next().unwrap() as usize;
    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = ints.next().unwrap() as usize;
    }
    
    println!("{}", Solution::solve(a));
}
