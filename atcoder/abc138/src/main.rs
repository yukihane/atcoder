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

    use std::io::*;

    pub fn answer() {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();

        let mut iter = buf.split_whitespace();
        let _n = iter.next().unwrap().parse::<i32>().unwrap();
        let v = iter
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let res = calc(&v);
        println!("{}", res);
    }

    fn calc(v: &Vec<i32>) -> f64 {
        let mut nums = v.clone();
        nums.sort();
        let mut prev = nums[0] as f64;
        for i in 1..nums.len() {
            prev = (prev + nums[i] as f64) / 2.0;
        }
        prev
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test1() {
            assert_eq!(3.5, calc(&vec![3, 4]));
        }
        #[test]
        fn test2() {
            assert_eq!(375.0, calc(&vec![500, 300, 200]));
        }
        #[test]
        fn test3() {
            assert_eq!(138.0, calc(&vec![138, 138, 138, 138, 138]));
        }
    }
}
mod b {
    use std::io::*;

    pub fn answer() {
        let mut buf = String::new();

        stdin().read_to_string(&mut buf).unwrap();
        let mut ite = buf.split_whitespace();

        // 1行目読み捨て
        ite.next();

        let seq = ite.map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();

        let res = calc(&seq);

        println!("{}", res);
    }
    fn calc(a: &Vec<i32>) -> f64 {
        let mut bunbo_neta = 1f64;
        for i in 0..a.len() {
            bunbo_neta *= a[i] as f64;
        }
        let mut bunbo = 0f64;
        for i in 0..a.len() {
            bunbo += bunbo_neta / (a[i] as f64);
        }

        let bunshi: f64 = bunbo_neta;

        bunshi / bunbo
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test1() {
            assert_eq!(calc(&vec![10, 30]), 7.5);
        }
        #[test]
        fn test2() {
            let res = calc(&vec![200, 200, 200]);
            assert!(res > 66.6666 && res < 66.67);
        }
        #[test]
        fn test_b_3() {
            let res = calc(&vec![1000]);
            assert_eq!(res, 1000.0);
        }
    }
}
mod a {
    use std::io::*;

    pub fn answer() {
        let mut buf = String::new();

        stdin().read_to_string(&mut buf).unwrap();

        let mut iter = buf.split_whitespace();

        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let s = iter.next().unwrap();

        let res = calc(a, s);
        println!("{}", res);
    }

    fn calc(a: i32, s: &str) -> &str {
        if a >= 3200 {
            return s;
        }
        "red"
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test1() {
            assert_eq!("pink", calc(3200, "pink"));
        }

        #[test]
        fn test2() {
            assert_eq!("red", calc(3199, "pink"));
        }

        #[test]
        fn test3() {
            assert_eq!("red", calc(4049, "red"));
        }

    }
}
