fn main() {
    println!("Hello, world!");
}

fn calc(input: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test1() {
        assert_eq!(calc(&vec![3, 1]), 2);
    }
    #[test]
    fn test2() {
        assert_eq!(calc(&vec![2, 7, 4]), 5);
    }
    #[test]
    fn test3() {
        assert_eq!(calc(&vec![20, 18, 2, 18]), 18);
    }

}
