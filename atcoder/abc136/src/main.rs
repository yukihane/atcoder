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
