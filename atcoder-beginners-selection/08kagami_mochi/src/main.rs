use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // 1行目読み捨て
    lines.next();

    let nums = lines
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let res = calc(&nums);

    println!("{}", res);
}

fn calc(input: &Vec<i32>) -> i32 {
    let uniq = input.into_iter().map(|x| *x).collect::<HashSet<i32>>();
    uniq.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tes1() {
        assert_eq!(calc(&vec![10, 8, 8, 6]), 3);
    }
    #[test]
    fn tes2() {
        assert_eq!(calc(&vec![15, 15, 15]), 1);
    }
    #[test]
    fn tes3() {
        assert_eq!(calc(&vec![50, 30, 50, 100, 50, 80, 30]), 4);
    }
}
