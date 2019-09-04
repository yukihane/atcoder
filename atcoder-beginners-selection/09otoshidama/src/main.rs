use std::fmt;
use std::io::{self, BufRead};

struct Input {
    // お札の枚数
    count: i32,
    // 合計金額
    total: i32,
}

#[derive(PartialEq, Debug)]
struct Output {
    // 1万円札
    ten: i32,
    // 五千円札
    five: i32,
    // 千円札
    one: i32,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.ten, self.five, self.one)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let line = lines.next().unwrap().unwrap();
    let mut nums = line.split_whitespace();
    let n = nums.next().unwrap().parse::<i32>().unwrap();
    let y = nums.next().unwrap().parse::<i32>().unwrap();
    let input = Input { count: n, total: y };
    let res = calc(&input);

    println!("{}", res);
}

fn calc(input: &Input) -> Output {
    for x in (0..(input.count + 1)).rev() {
        let total = totalize(x, 0, 0);
        if total > input.total {
            continue;
        }
        let remain = input.count - x;
        for y in (0..(remain + 1)).rev() {
            let total = totalize(x, y, remain - y);
            if total == input.total {
                return Output {
                    ten: x,
                    five: y,
                    one: remain - y,
                };
            }
        }
    }

    Output {
        ten: -1,
        five: -1,
        one: -1,
    }
}

fn totalize(ten: i32, five: i32, one: i32) -> i32 {
    10000 * ten + 5000 * five + 1000 * one
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let output = calc(&Input {
            count: 9,
            total: 45000,
        });
        assert_eq!(
            output,
            Output {
                ten: 4,
                five: 0,
                one: 5,
            }
        );
    }

    #[test]
    fn test2() {
        let output = calc(&Input {
            count: 20,
            total: 196000,
        });
        assert_eq!(
            output,
            Output {
                ten: -1,
                five: -1,
                one: -1,
            }
        );
    }

    #[test]
    fn test3() {
        let output = calc(&Input {
            count: 1000,
            total: 1234000,
        });
        assert_eq!(
            output,
            Output {
                ten: 26,
                five: 0,
                one: 974,
            }
        );
    }

    #[test]
    fn test4() {
        let output = calc(&Input {
            count: 2000,
            total: 20000000,
        });
        assert_eq!(
            output,
            Output {
                ten: 2000,
                five: 0,
                one: 0,
            }
        );
    }
}
