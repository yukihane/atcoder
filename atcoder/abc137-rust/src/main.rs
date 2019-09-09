fn main() {
    let mode = "b";
    if mode == "a" {
        a::answer();
    } else if mode == "b" {
        b::answer();
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
