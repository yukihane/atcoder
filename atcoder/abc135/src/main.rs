fn main() {
    let mode = "a";
    if mode == "a" {
        a::solve();
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
        match calc(a, b) {
            None => println!("IMPOSSIBLE"),
            Some(x) => println!("{}", x),
        }
    }

    fn calc(a: i32, b: i32) -> Option<i32> {
        let total = a + b;
        if total & 0x1 == 0x1 {
            return None;
        }
        Some(total / 2)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test1() {
            assert_eq!(calc(2, 16), Some(9));
        }
    }
}
