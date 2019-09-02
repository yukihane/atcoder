fn main() {
    println!("Hello, world!");
}

fn calc(input: &mut Vec<i32>) -> i32 {
    input.sort_by_key(|x| x * -1);

    let foo = vec![1, 35, 64, 36, 26];
    for (i, item) in foo.iter().enumerate() {
        println!("The {}th item is {}", i + 1, item);
    }

    //0:alice, 1:bob
    let mut points = [0; 2];
    for (i, item) in input.iter().enumerate() {
        points[i % 2] += item;
    }
    points[0] - points[1]
}

#[cfg(test)]
mod tests {
    use crate::*;
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
