// 文字列をtoken化
pub fn tokenize(program: &str) -> Vec<String> {
    program
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::tokenize;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize::tokenize("()"), ["(", ")"]);
        assert_eq!(tokenize::tokenize("(1)"), ["(", "1", ")"]);
        assert_eq!(tokenize::tokenize("(+ 1 1)"), ["(", "+", "1", "1", ")"]);
    }

}