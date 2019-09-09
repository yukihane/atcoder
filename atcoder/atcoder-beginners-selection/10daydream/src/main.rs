use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let text = lines.next().unwrap().unwrap();
    match calc(&text) {
        true => println!("YES"),
        false => println!("NO"),
    }
}

fn calc(text: &str) -> bool {
    let mut input = text;
    loop {
        if input.len() == 0 {
            return true;
        }
        match next(
            &input,
            &vec!["dream", "dreamer", "erase", "eraser"],
            &vec!["dream", "erase"],
        ) {
            Ok(res) => input = res,
            Err(_) => return false,
        }
    }
}

fn next<'a>(text: &'a str, begin: &Vec<&str>, next: &Vec<&str>) -> Result<&'a str, &'static str> {
    for b in begin {
        if text.starts_with(b) {
            let follow_text = &text[b.len()..];
            if follow_text.len() == 0 {
                return Ok("");
            }
            for n in next {
                if follow_text.starts_with(n) {
                    return Ok(&text[b.len()..]);
                }
            }
        }
    }
    Err("not match")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(calc("erasedream"), true);
    }
    #[test]
    fn test2() {
        assert_eq!(calc("dreameraser"), true);
    }
    #[test]
    fn test3() {
        assert_eq!(calc("dreamerer"), false);
    }
}
