fn main() {
    let text = "Hello, world!".chars().collect::<Vec<char>>();
    println!("{:?}", text);
    calc("erasedream");
}

fn calc(text: &str) -> bool {
    let mut input = text;
    loop {
        // println!("{}", input);
        if input.len() == 0 {
            return true;
        } else if input.len() < 5 {
            // 今回の単語は最小でも文字数5
            return false;
        } else if input.starts_with("dream") {
            match pop_dream(input) {
                Ok(res) => input = res,
                Err(_) => return false,
            }
        } else if input.starts_with("erase") {
            match pop_erase(input) {
                Ok(res) => input = res,
                Err(_) => return false,
            }
        } else {
            return false;
        }
    }
}

fn pop_dream(text: &str) -> Result<&str, &'static str> {
    // println!("pop_dream");
    if text == "dream" || text == "dreamer" {
        // println!("OK!");
        return Ok("");
    }
    let skipped = &text["dream".len()..];
    if skipped.starts_with("d") || skipped.starts_with("era") {
        // println!("x: {}", skipped);
        return Ok(skipped);
    }
    let skipped = &text["dreamer".len()..];
    if skipped.starts_with("d") || skipped.starts_with("e") {
        // println!("y: {}", skipped);
        return Ok(skipped);
    }
    Err("not match")
}

fn pop_erase(text: &str) -> Result<&str, &'static str> {
    if text == "erase" || text == "eraser" {
        return Ok("");
    }
    let skipped = &text["erase".len()..];
    if skipped.starts_with("d") || skipped.starts_with("e") {
        return Ok(skipped);
    }
    let skipped = &text["eraser".len()..];
    if skipped.starts_with("d") || skipped.starts_with("e") {
        return Ok(skipped);
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
