use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    let line3 = iterator.next().unwrap().unwrap();

    let a = line1.parse::<i32>().unwrap();

    let mut w2 = line2.split_whitespace();
    let b = w2.next().unwrap().parse::<i32>().unwrap();
    let c = w2.next().unwrap().parse::<i32>().unwrap();

    let text = line3;

    println!("{} {}", a + b + c, text);
}
