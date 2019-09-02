use std::io::{self, BufRead};

struct Sample {
    numbers: Vec<i32>,
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
    let sample = Sample { numbers: nums };

    let res = calc(&sample);
    println!("{}", res);
}

fn calc(sample: &Sample) -> i32 {
    let mut nums = sample.numbers.clone();

    let mut times = 0;
    loop {
        // println!("times: {}, values: {:?}", times, nums);
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
