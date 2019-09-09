use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut ite = stdin.lock().lines();
    // 1行目読み捨て
    ite.next();
    let str = ite.next().unwrap().unwrap();
    let mut input = str
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let res = calc(&mut input);

    println!("{}", res);
}

fn calc(input: &mut Vec<i32>) -> i32 {
    input.sort_by_key(|x| x * -1);

    //0:alice, 1:bob
    let mut points = [0; 2];
    for (i, item) in input.iter().enumerate() {
        points[i % 2] = points[i % 2] + item;
    }
    points[0] - points[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(calc(&mut vec![3, 1]), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(calc(&mut vec![2, 7, 4]), 5);
    }
    #[test]
    fn test3() {
        assert_eq!(calc(&mut vec![20, 18, 2, 18]), 18);
    }

}
