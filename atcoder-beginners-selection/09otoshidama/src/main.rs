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

fn main() {
    println!("Hello, world!");
}

fn calc(input: &Input) -> Output {
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
                ten: 0,
                five: 9,
                one: 0
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
