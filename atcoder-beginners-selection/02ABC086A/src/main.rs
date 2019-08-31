use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut ite = stdin.lock().lines();
    let line = ite.next().unwrap().unwrap();
    let mut nums = line.split_whitespace();

    let a = nums.next().unwrap().parse::<i32>().unwrap();
    let b = nums.next().unwrap().parse::<i32>().unwrap();

    let res = a * b % 2;
    if res == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}
