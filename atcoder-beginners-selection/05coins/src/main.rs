use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut ite = stdin.lock().lines();
    let c500 = ite.next().unwrap().unwrap().parse::<i32>().unwrap();
    let c100 = ite.next().unwrap().unwrap().parse::<i32>().unwrap();
    let c50 = ite.next().unwrap().unwrap().parse::<i32>().unwrap();
    let total = ite.next().unwrap().unwrap().parse::<i32>().unwrap();

    println!("500:{}, 100:{},50:{}, total:{}", c500, c100, c50, total);
}
