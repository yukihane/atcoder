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
    use std::io::BufRead;

    pub fn solve() {
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines();

        lines.next();

        let a = lines
            .next()
            .map(|x| {
                x.unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .unwrap();
        //        println!("a: {:?}", a);

        let b = lines
            .next()
            .map(|x| {
                x.unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect()
            })
            .unwrap();
        //        println!("b: {:?}", b);

        let res = calc(a, b);
        println!("{}", res);
    }

    fn calc(mut a: Vec<i32>, mut b: Vec<i32>) -> u64 {
        let mut total_killed = 0u64;
        for i in 0..b.len() {
            let killed = std::cmp::min(a[i], b[i]);
            total_killed += killed as u64;
            a[i] -= killed;
            b[i] -= killed;

            let killed = std::cmp::min(a[i + 1], b[i]);
            total_killed += killed as u64;
            a[i + 1] -= killed;
            b[i] -= killed;
        }
        total_killed
    }
}

mod b {
    use std;
    use std::io::Read;

    pub fn solve() {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();

        let mut text = buf.split_whitespace();
        text.next();

        let input: Vec<i32> = text.map(|x| x.parse().unwrap()).collect();
        let res = if calc(input) { "YES" } else { "NO" };
        println!("{}", res);
    }
    fn calc(mut p: Vec<i32>) -> bool {
        let ng_point = match search_ng(&p) {
            Some(x) => x,
            None => return true,
        };
        let min_point = match search_min(&p, ng_point + 1) {
            Some(x) => x,
            None => return true,
        };
        p.swap(ng_point, min_point);
        is_sorted(p)
    }
    fn is_sorted(p: Vec<i32>) -> bool {
        for i in 0..(p.len() - 1) {
            if p[i] > p[i + 1] {
                return false;
            }
        }
        true
    }

    fn search_ng(p: &Vec<i32>) -> Option<usize> {
        for i in 0..(p.len() - 1) {
            if p[i] > p[i + 1] {
                return Some(i);
            }
        }
        None
    }

    fn search_min(p: &Vec<i32>, start: usize) -> Option<usize> {
        let mut min = std::i32::MAX;
        let mut point = None;
        for i in start..p.len() {
            if p[i] < min {
                min = p[i];
                point = Some(i);
            }
        }
        point
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
