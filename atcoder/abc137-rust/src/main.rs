fn main() {
    let mode = "c";
    if mode == "a" {
        a::answer();
    } else if mode == "b" {
        b::answer();
    } else if mode == "c" {
        c::answer();
    }
}

mod c {
    use std;
    use std::io::Read;

    pub fn answer() {
        let mut buff = String::new();
        std::io::stdin().read_to_string(&mut buff).unwrap();
        let mut text = buff.split_whitespace();
        text.next();
        let input: Vec<&str> = text.into_iter().collect();

        let res = calc(&input);
        println!("{}", res);
    }

    fn calc(input: &Vec<&str>) -> u64 {
        let sorted_str: Vec<String> = input.iter().map(|x| sort_string(x)).collect();
        let sorted_text = sort_text(&sorted_str);
        count(&sorted_text)
    }

    fn count(input: &Vec<String>) -> u64 {
        let mut count = 0u64;
        let mut stack = input.clone();

        loop {
            let elm = stack.pop();
            match elm {
                None => break,
                Some(x) => {
                    let sames = remove_sames(&x, &mut stack);
                    count += sigma(sames);
                }
            }
        }
        count
    }

    fn sigma(number: u64) -> u64 {
        (1..(number + 1)).fold(0, |a, b| a + b)
    }

    fn remove_sames(text: &str, stack: &mut Vec<String>) -> u64 {
        let mut count = 0;

        loop {
            match stack.pop() {
                None => break,
                Some(x) => {
                    if &x == text {
                        count += 1;
                    } else {
                        stack.push(x);
                        break;
                    }
                }
            }
        }

        count
    }

    fn sort_text(input: &Vec<String>) -> Vec<String> {
        let mut res = input.clone();
        res.sort();
        res
    }

    fn sort_string(input: &str) -> String {
        let mut chars: Vec<_> = input.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        chars.into_iter().collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test1() {
            let input = vec!["acornistnt", "peanutbomb", "constraint"];
            assert_eq!(calc(&input), 1u64);
        }
        #[test]
        fn test2() {
            let input = vec!["oneplustwo", "ninemodsix"];
            assert_eq!(calc(&input), 0u64);
        }
        #[test]
        fn test3() {
            let input = vec![
                "abaaaaaaaa",
                "oneplustwo",
                "aaaaaaaaba",
                "twoplusone",
                "aaaabaaaaa",
            ];
            assert_eq!(calc(&input), 4u64);
        }
    }
}
mod b {
    use std;
    use std::io::Read;

    pub fn answer() {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        let mut ite = buf.split_whitespace();
        let k = ite.next().unwrap().parse::<i32>().unwrap();
        let x = ite.next().unwrap().parse::<i32>().unwrap();
        let res = calc(k, x);
        let text = res
            .iter()
            .fold(String::new(), |acc, &num| acc + &num.to_string() + " ");
        println!("{}", text);
    }

    fn calc(k: i32, x: i32) -> Vec<i32> {
        let left = std::cmp::max(x - (k - 1), -1000000);
        let right = x + (k - 1);

        (left..(right + 1)).map(i32::from).collect::<Vec<_>>()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test1() {
            assert_eq!(calc(3, 7), vec![5, 6, 7, 8, 9]);
        }
        #[test]
        fn test2() {
            assert_eq!(calc(4, 0), vec![-3, -2, -1, 0, 1, 2, 3]);
        }
        #[test]
        fn test3() {
            assert_eq!(calc(1, 100), vec![100]);
        }
    }
}

mod a {
    use std;
    use std::io::Read;

    pub fn answer() {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();

        let mut ite = buf.split_whitespace();
        let a = ite.next().unwrap().parse::<i32>().unwrap();
        let b = ite.next().unwrap().parse::<i32>().unwrap();

        let res = calc(a, b);
        println!("{}", res);
    }
    fn calc(a: i32, b: i32) -> i32 {
        let tasu = a + b;
        let hiku = a - b;
        let kakeru = a * b;
        std::cmp::max(tasu, std::cmp::max(hiku, kakeru))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test1() {
            assert_eq!(calc(-13, 3), -10);
        }
        #[test]
        fn test2() {
            assert_eq!(calc(1, -33), 34);
        }
        #[test]
        fn test3() {
            assert_eq!(calc(13, 3), 39);
        }
    }

}
