use std::ops::Range;

use regex::Match;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerMatch {
    text: String,
    start: usize,
    end: usize,
}

impl LexerMatch {
    pub fn new(text: &str, start: usize, end: usize) -> Self {
        Self {
            text: text.to_owned(),
            start,
            end,
        }
    }

    pub fn new_match(text: &str, mat: &Match) -> Self {
        LexerMatch::new(text, mat.start(), mat.end())
    }

    pub fn new_full_match(text: &str) -> Self {
        LexerMatch::new(text, 0, text.len())
    }

    pub fn new_eof(text: &str) -> Self {
        let start = if text.len() > 0 { text.len() - 1 } else { 0 };
        LexerMatch::new(text, start, text.len())
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn as_str(&self) -> &str {
        &self.text[self.range()]
    }
}
