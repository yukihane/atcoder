struct Record {
    time: u32,
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");
}

fn calc(records: &Vec<Record>) -> bool {
    false
}

fn can_checkin(current: &Record, next: &Record) -> bool {
    let distance = measure(current, next);
    false
}

fn measure(p1: &Record, p2: &Record) -> i32 {
    let x = (p1.x - p2.x).abs();
    let y = (p1.y - p2.y).abs();
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_measure1() {
        let distance = measure(
            &Record {
                time: 0,
                x: 0,
                y: 0,
            },
            &Record {
                time: 3,
                x: 1,
                y: 2,
            },
        );
        assert_eq!(distance, 3);
    }
    #[test]
    fn test_measure2() {
        let distance = measure(
            &Record {
                time: 0,
                x: 10,
                y: 0,
            },
            &Record {
                time: 3,
                x: 1,
                y: 2,
            },
        );
        assert_eq!(distance, 11);
    }
    #[test]
    fn test_checkin1() {
        assert!(can_checkin(
            &Record {
                time: 0,
                x: 0,
                y: 0,
            },
            &Record {
                time: 3,
                x: 1,
                y: 2,
            },
        ));
    }
}
