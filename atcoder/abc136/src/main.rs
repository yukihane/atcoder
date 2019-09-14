fn main() {
    let mode = "b";
    if mode == "a" {
        a::solve();
    } else if mode == "b" {
        b::solve();
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
