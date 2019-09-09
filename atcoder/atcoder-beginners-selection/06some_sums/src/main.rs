use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut ite = stdin.lock().lines();
    let numsstr = ite.next().unwrap().unwrap();
    let mut nums = numsstr.split_whitespace();

    let n = nums.next().unwrap().parse::<i32>().unwrap();
    let a = nums.next().unwrap().parse::<i32>().unwrap();
    let b = nums.next().unwrap().parse::<i32>().unwrap();

    let mut total = 0;
    for x in 1..(n + 1) {
        let sum = calc(x);
        if a <= sum && b >= sum {
            total += x;
        }
    }
    println!("{}", total);
}

fn calc(x: i32) -> i32 {
    let mut sum = 0;
    calc_internal(x, &mut sum);
    sum
}

fn calc_internal(target: i32, sum: &mut i32) {
    if target == 0 {
        return;
    }
    let i = target % 10;
    *sum = *sum + i;
    calc_internal(target / 10, sum);
}
