use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let chars = stdin.lock().lines().next().unwrap().unwrap();
    let ite = chars.chars();
    let res = ite.filter(|x| *x == '1').count();

    println!("{}", res);
}
