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
    false
}

#[cfg(test)]
mod tests {
    use super::*;

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
