pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            if let Err(e) = writeln!(writer, "{}", line) {
                eprintln!("Couldn't write to stdout: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matches() {
        let mut result = Vec::new();
        find_matches("lorem ipsum\n", "lorem", &mut result);
        assert_eq!(result, b"lorem ipsum\n");
    }
}
