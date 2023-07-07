use anyhow::{Context, Result};
use std::fs;

/// Parses the text in a file
pub fn parse_input(path: &str) -> Result<String> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read file from {}", path))?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input_from_file() {
        let result = parse_input("test.txt").unwrap();
        assert_eq!(result, String::from("test\n"));
    }

    #[test]
    fn returns_error_when_file_is_missing() {
        let error = parse_input("../test.txt").unwrap_err();
        assert_eq!(error.to_string(), String::from("Failed to read file from ../test.txt"));
    }
}
