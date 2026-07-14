/// A single non-blank line of input, tagged with its position in the
/// original input so that ordering can be preserved after clustering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub text: String,
    pub original_index: usize,
}

/// Splits `input` into lines, skipping any that are blank or contain only
/// whitespace. The `original_index` on each returned `Line` reflects its
/// position in the original (unfiltered) input, so relative ordering between
/// lines is preserved even though blank lines are dropped.
pub fn read_non_blank_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .enumerate()
        .filter(|(_, text)| !text.trim().is_empty())
        .map(|(original_index, text)| Line {
            text: text.to_string(),
            original_index,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_blank_lines() {
        let input = "first\n\nsecond\n   \nthird";
        let lines = read_non_blank_lines(input);
        let texts: Vec<&str> = lines.iter().map(|line| line.text.as_str()).collect();
        assert_eq!(texts, vec!["first", "second", "third"]);
    }

    #[test]
    fn preserves_original_index() {
        let input = "first\n\nsecond";
        let lines = read_non_blank_lines(input);
        assert_eq!(lines[0].original_index, 0);
        assert_eq!(lines[1].original_index, 2);
    }

    #[test]
    fn empty_input_yields_no_lines() {
        assert!(read_non_blank_lines("").is_empty());
    }
}
