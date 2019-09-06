struct Record {
    time: i32,
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
    let time = next.time - current.time;
    let remain_time = time - distance;
    if remain_time < 0 || remain_time % 2 != 0 {
        return false;
    }
    true
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
    #[test]
    fn test_checkin2() {
        assert!(can_checkin(
            &Record {
                time: 3,
                x: 1,
                y: 2,
            },
            &Record {
                time: 6,
                x: 1,
                y: 1
            },
        ));
    }
}
