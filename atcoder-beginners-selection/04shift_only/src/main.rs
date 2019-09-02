use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Sample<'a> {
    numbers: &'a Vec<i32>,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    // 1行目読み捨て
    lines.next();

    let str = lines.next().unwrap().unwrap();
    let nums = str
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let sample = Sample { numbers: &nums };

    let res = calc(&sample);
    println!("{}", res);
}

fn calc(sample: &Sample) -> i32 {
    let mut nums = sample.numbers.clone();

    let mut times = 0;
    loop {
        println!("times: {}, values: {:?}", times, nums);
        let res = half(&nums);
        match res {
            Ok(v) => {
                nums = v;
                times = times + 1;
            }
            Err(_) => break,
        };
    }
    times
}

fn half(nums: &Vec<i32>) -> Result<Vec<i32>, &'static str> {
    nums.iter()
        .map(|x| {
            if x % 2 != 0 {
                return Err("odd");
            }
            Ok(x / 2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test1() {
        let data = [(vec![8, 12, 40], 2)];
        for x in &data {
            let input = Sample { numbers: &x.0 };
            println!("input: {:?}", input);
            let result = calc(&input);
            println!("result: {}", result);
            assert_eq!(result, x.1);
        }
    }
    #[test]
    fn test2() {
        let input = Sample {
            numbers: &vec![5, 6, 8, 10],
        };
        let result = calc(&input);
        assert_eq!(result, 0);
    }
    #[test]
    fn test3() {
        let input = Sample {
            numbers: &vec![
                382253568, 723152896, 37802240, 379425024, 404894720, 471526144,
            ],
        };
        let result = calc(&input);
        assert_eq!(result, 8);
    }
}
