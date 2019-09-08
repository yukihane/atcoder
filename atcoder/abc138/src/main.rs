fn main() {
    let mode = "a";

    if mode == "a" {
        a::answer();
    }
}

mod a {
    use std::io::Read;

    pub fn answer() {
        let mut buf = String::new();

        std::io::stdin().read_to_string(&mut buf).unwrap();

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
