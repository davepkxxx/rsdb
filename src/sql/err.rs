use regex::Regex;

use super::lexer::mat::LexerMatch;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxError {
    pub cause: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

impl SyntaxError {
    pub fn new_lexer(mat: LexerMatch, cause: &str) -> SyntaxError {
        SyntaxError {
            cause: cause.to_owned(),
            text: mat.text().to_owned(),
            start: mat.start(),
            end: mat.end(),
        }
    }

    pub fn new_excpeted(mat: LexerMatch) -> SyntaxError {
        SyntaxError {
            cause: format!("excpeted {}", mat.as_str()),
            text: mat.text().to_owned(),
            start: mat.start(),
            end: mat.end(),
        }
    }

    pub fn new_missing(mat: LexerMatch, name: &str) -> SyntaxError {
        SyntaxError {
            cause: format!("missing {}", name),
            text: mat.text().to_owned(),
            start: mat.start(),
            end: mat.end(),
        }
    }

    fn point(&self) -> (usize, usize) {
        let regex = Regex::new(r"^(\r\n)|[\r\n]").unwrap();
        let mut row = 0_usize;
        let mut col = 0_usize;
        let mut i = 0_usize;

        while i < self.text.len() {
            match regex.find_at(&self.text, i) {
                Some(mat) => {
                    row += 1;
                    col = 0;
                    i += mat.range().len();
                }
                None => {
                    col += 1;
                    i += 1;
                }
            }
        }

        (row, col)
    }

    pub fn msg(&self) -> String {
        let (row, col) = self.point();
        format!("SyntaxError: {} [{}, {}]", self.cause, row, col)
    }
}
