

fn tokenize(program: &str) -> Vec<String> {
    program
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::tokenize;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("()"), ["(", ")"]);
        assert_eq!(tokenize("(1)"), ["(", "1", ")"]);
        assert_eq!(tokenize("(+ 1 1)"), ["(", "+", "1", "1", ")"]);
    }
}