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
    let max_ten = raise(input.total, 10000, input.count);
    for i in (0..(max_ten + 1)).rev() {
        let mut counter = Output {
            ten: 0,
            five: 0,
            one: 0,
        };
        counter.ten = i;

        let remain_total = input.total - totalize(&counter);
        let remain_count = input.count - (counter.ten + counter.five + counter.one);
        let max_five = raise(remain_total, 5000, remain_count);
        for j in (0..(max_five + 1)).rev() {
            counter.five = j;

            let remain_total = input.total - totalize(&counter);
            let remain_count = input.count - (counter.ten + counter.five + counter.one);
            let max_one = raise(remain_total, 1000, remain_count);

            counter.one = max_one;
            let remain_total = input.total - totalize(&counter);
            let remain_count = input.count - (counter.ten + counter.five + counter.one);
            if remain_total == 0 && remain_count == 0 {
                return counter;
            }
        }
    }
    for ten in 0..(2000 + 1) {
        if ten * 10000 > input.total {
            break;
        } else if ten > input.count {
            break;
        }
        for five in 0..(2000 + 1) {
            if ten * 10000 + five * 5000 > input.total {
                break;
            } else if ten + five > input.count {
                break;
            }
            for one in 0..(2000 + 1) {
                let actual = 10000 * ten + 5000 * five + 1000 * one;
                if actual > input.total {
                    break;
                } else if ten + five + one > input.count {
                    break;
                } else if actual == input.total && ten + five + one == input.count {
                    return Output {
                        ten: ten,
                        five: five,
                        one: one,
                    };
                }
            }
        }
    }
    Output {
        ten: -1,
        five: -1,
        one: -1,
    }
}

fn totalize(nums: &Output) -> i32 {
    10000 * nums.ten + 5000 * nums.five + 1000 * nums.one
}

// ceil: 求めたい上限値
// unit: 硬貨の単価
// max: 所持している硬貨の枚数
fn raise(ceil: i32, unit: i32, max: i32) -> i32 {
    for x in 0..(max + 1) {
        let total = unit * x;
        if ceil == total {
            return x;
        } else if ceil < total {
            return x - 1;
        }
    }
    max
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
                one: 5
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
                one: -1
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
                ten: 2,
                five: 54,
                one: 944
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
                one: 0
            }
        );
    }

}
