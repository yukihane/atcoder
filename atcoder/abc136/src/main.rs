fn main() {
    let mode = "c";
    if mode == "a" {
        a::solve();
    } else if mode == "b" {
        b::solve();
    } else if mode == "c" {
        c::solve();
    }
}
mod c {
    use std;
    use std::io::Read;

    pub fn solve() {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        let mut text = buf.split_whitespace();
        let n = text.next().unwrap().parse().unwrap();
        let data = text.into_iter().map(|x| x.parse().unwrap()).collect();

        let res = match calc(n, data) {
            true => "Yes",
            false => "No",
        };

        println!("{}", res);
    }
    fn calc(n: usize, mut h: Vec<i32>) -> bool {
        for i in (0..(n - 1)).rev() {
            match h[i] > h[i + 1] {
                false => {}
                true => {
                    h[i] -= 1;
                    if h[i] > h[i + 1] {
                        return false;
                    }
                }
            }
        }

        true
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test1() {
            assert_eq!(true, calc(5, vec![1, 2, 1, 1, 3]));
        }
        #[test]
        fn test2() {
            assert_eq!(false, calc(4, vec![1, 3, 2, 1]));
        }
        #[test]
        fn test3() {
            assert_eq!(true, calc(4, vec![1, 2, 3, 4, 5]));
        }
        #[test]
        fn test4() {
            assert_eq!(true, calc(1, vec![1000000000]));
        }
    }
}
mod b {
    use std;
    use std::io::Read;

    pub fn solve() {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        let n = buf.split_whitespace().next().unwrap().parse().unwrap();
        let res = calc(n);
        println!("{}", res);
    }

    fn calc(num: i32) -> i32 {
        let mut base = 10;
        let mut res = 0;
        loop {
            if num >= base {
                res += base - base / 10;
                base *= 100;
            } else {
                break;
            }
        }

        let amari = match num - (base / 10) >= 0 {
            true => num - (base / 10) + 1,
            false => 0,
        };

        res + amari
    }
}

mod a {
    use std;
    use std::io::Read;

    pub fn solve() {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        let mut text = buf.split_whitespace();
        let a = text.next().unwrap().parse().unwrap();
        let b = text.next().unwrap().parse().unwrap();
        let c = text.next().unwrap().parse().unwrap();

        let res = calc(a, b, c);
        println!("{}", res);
    }

    fn calc(a: i32, b: i32, c: i32) -> i32 {
        let used = std::cmp::min(a - b, c);
        c - used
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test1() {}
    }

}
