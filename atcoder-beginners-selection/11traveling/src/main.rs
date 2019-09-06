struct Record {
    time: i32,
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");
}

fn calc(records: &Vec<Record>) -> bool {
    let size = records.len();
    for i in 0..(size - 1) {
        let current = &records[i];
        let next = &records[i + 1];
        if !can_checkin(current, next) {
            return false;
        }
    }
    true
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
    fn test1() {
        let points = vec![
            Record {
                time: 0,
                x: 0,
                y: 0,
            },
            Record {
                time: 3,
                x: 1,
                y: 2,
            },
            Record {
                time: 6,
                x: 1,
                y: 1,
            },
        ];

        assert!(calc(&points));
    }
    #[test]
    fn test2() {
        let points = vec![
            Record {
                time: 0,
                x: 0,
                y: 0,
            },
            Record {
                time: 2,
                x: 100,
                y: 100,
            },
        ];
        assert!(!calc(&points));
    }
    #[test]
    fn test3() {
        let points = vec![
            Record {
                time: 0,
                x: 0,
                y: 0,
            },
            Record {
                time: 5,
                x: 1,
                y: 1,
            },
            Record {
                time: 100,
                x: 1,
                y: 1,
            },
        ];
        assert!(!calc(&points));
    }
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
