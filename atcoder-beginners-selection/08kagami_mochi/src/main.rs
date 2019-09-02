use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn calc(input: &Vec<i32>) -> i32 {
    let uniq = input.into_iter().map(|x| *x).collect::<HashSet<i32>>();
    uniq.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tes1() {
        assert_eq!(calc(&vec![10, 8, 8, 6]), 3);
    }
    #[test]
    fn tes2() {
        assert_eq!(calc(&vec![15, 15, 15]), 1);
    }
    #[test]
    fn tes3() {
        assert_eq!(calc(&vec![50, 30, 50, 100, 50, 80, 30]), 4);
    }
}
