fn main() {
    let text = "Hello, world!".chars().collect::<Vec<char>>();
    println!("{:?}", text);
    calc("Hello, world!");
}

fn calc(text: &str) -> bool {
    let mut input = text;
    loop {
        if input.len() == 0 {
            return true;
        } else if input.len() < 5 {
            // 今回の単語は最小でも文字数5
            return false;
        } else if input.starts_with("dream") {
            match pop_dream(text) {
                Ok(res) => input = res,
                Err(_) => return false,
            }
        } else if input.starts_with("erase") {
            // TODO
        } else {
            return false;
        }
    }
}

fn pop_dream(text: &str) -> Result<&str, String> {
    // TODO
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(calc("erasedream"), "YES");
    }
}
