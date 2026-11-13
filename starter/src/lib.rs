//! A small domain model with validation at its boundary.
//!
//! ```
//! use signal_notebook::{parse_scores, mean};
//! let scores = parse_scores("25\n75\n")?;
//! assert_eq!(mean(&scores), Some(50.0));
//! # Ok::<(), signal_notebook::ParseScoreError>(())
//! ```

use std::fmt;

/// A validated integer score from 0 through 100, inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Score(u8);

impl Score {
    /// Returns `None` if the value is outside the domain.
    pub fn new(value: u8) -> Option<Self> {
        (value <= 100).then_some(Self(value))
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseScoreError {
    pub line: usize,
}

impl fmt::Display for ParseScoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: expected an integer from 0 to 100", self.line)
    }
}

impl std::error::Error for ParseScoreError {}

/// Parses one score per nonblank line; preserves original line numbers in errors.
///
/// # Errors
/// Returns the first invalid line. Blank lines are ignored.
pub fn parse_scores(input: &str) -> Result<Vec<Score>, ParseScoreError> {
    input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let text = line.trim();
            if text.is_empty() {
                None
            } else {
                Some(
                    text.parse::<u8>()
                        .ok()
                        .and_then(Score::new)
                        .ok_or(ParseScoreError { line: index + 1 }),
                )
            }
        })
        .collect()
}

/// Returns `None` for an empty slice, instead of producing a misleading zero.
pub fn mean(scores: &[Score]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }
    Some(
        scores
            .iter()
            .map(|score| f64::from(score.value()))
            .sum::<f64>()
            / scores.len() as f64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_boundaries() {
        assert_eq!(Score::new(0).map(Score::value), Some(0));
        assert_eq!(Score::new(100).map(Score::value), Some(100));
        assert_eq!(Score::new(101), None);
        assert_eq!(Score::new(255), None);
    }

    #[test]
    fn parses_whitespace_and_crlf() {
        let scores = parse_scores(" 0 \r\n\r\n100\r\n").unwrap();
        assert_eq!(
            scores.iter().map(|s| s.value()).collect::<Vec<_>>(),
            [0, 100]
        );
        assert_eq!(mean(&scores), Some(50.0));
    }

    #[test]
    fn reports_physical_line_number() {
        assert_eq!(
            parse_scores("10\n\n101\n"),
            Err(ParseScoreError { line: 3 })
        );
    }

    #[test]
    fn rejects_non_domain_values() {
        for text in ["-1", "256", "1.5", "hello", "∞"] {
            assert!(parse_scores(text).is_err(), "accepted {text}");
        }
    }

    #[test]
    fn empty_has_no_mean() {
        assert_eq!(mean(&parse_scores(" \n\t").unwrap()), None);
    }

    #[test]
    fn error_is_actionable() {
        assert_eq!(
            ParseScoreError { line: 7 }.to_string(),
            "line 7: expected an integer from 0 to 100"
        );
    }
}
