use regex::Regex;

use crate::sql::err::SyntaxError;

use super::{lexer::Lexer, mat::LexerMatch};

#[derive(Debug, Clone)]
pub struct LexerPattern {
    text: String,
}

impl LexerPattern {
    const PATTERN_SELECT: &'static str = "^(?i)SELECT";
    const PATTERN_FROM: &'static str = "^(?i)FROM";
    const PATTERN_ALIAS: &'static str = "^(?i)AS";
    const PATTERN_STAR: &'static str = r"^\*";
    const PATTERN_COMMA: &'static str = "^,";
    const PATTERN_WHITESPACE: &'static str = r"^[\s]+";
    const PATTERN_NAME: &'static str = r"^[A-Za-z][\w]*";

    const PATTERNS: [&'static str; 7] = [
        Self::PATTERN_SELECT,
        Self::PATTERN_FROM,
        Self::PATTERN_ALIAS,
        Self::PATTERN_STAR,
        Self::PATTERN_COMMA,
        Self::PATTERN_WHITESPACE,
        Self::PATTERN_NAME,
    ];

    pub fn new(text: &str) -> Self {
        LexerPattern {
            text: text.to_owned(),
        }
    }

    fn into_lexer(re: Regex, value: LexerMatch) -> Option<Lexer> {
        match re.as_str() {
            Self::PATTERN_SELECT => Some(Lexer::SELECT(value)),
            Self::PATTERN_FROM => Some(Lexer::FROM(value)),
            Self::PATTERN_ALIAS => Some(Lexer::ALIAS(value)),
            Self::PATTERN_STAR => Some(Lexer::STAR(value)),
            Self::PATTERN_COMMA => Some(Lexer::COMMA(value)),
            Self::PATTERN_WHITESPACE => Some(Lexer::WHITESPACE(value)),
            Self::PATTERN_NAME => Some(Lexer::NAME(value)),
            _ => None,
        }
    }

    fn match_pattern(&self, text_index: usize, patterns_index: usize) -> Option<Lexer> {
        // get lexer
        match Self::PATTERNS.get(patterns_index) {
            // has lexer -> get regex -> match
            Some(expr) => {
                let re = Regex::new(expr).unwrap();
                match re.find_at(&self.text, text_index) {
                    // is match -> find next matches
                    Some(mat) => {
                        match self.match_pattern(text_index, patterns_index + 1) {
                            // has next matches
                            Some(next_lexer) => Some(
                                // return next lexer if its len longer or its is fixed
                                if next_lexer.value().range().len() > mat.range().len()
                                    || next_lexer.is_value_ignores()
                                {
                                    next_lexer.to_owned()
                                } else {
                                    LexerPattern::into_lexer(
                                        re,
                                        LexerMatch::new_match(self.text.as_str(), &mat),
                                    )
                                    .unwrap()
                                },
                            ),
                            None => Some(
                                LexerPattern::into_lexer(
                                    re,
                                    LexerMatch::new_match(self.text.as_str(), &mat),
                                )
                                .unwrap(),
                            ),
                        }
                    }
                    None => self.match_pattern(text_index, patterns_index + 1),
                }
            }
            None => None,
        }
    }

    pub fn match_text(&self, text_index: usize) -> Result<Vec<Lexer>, SyntaxError> {
        // no end
        if text_index < self.text.len() {
            // find lexer
            match self.match_pattern(text_index, 0) {
                // has lexer
                Some(lexer) => {
                    // match next lexer
                    match self.match_text(text_index + lexer.value().range().len()) {
                        // has lexers
                        Ok(lexers) => {
                            // insert lexer if it is not whitespace
                            if !matches!(lexer, Lexer::WHITESPACE(_)) {
                                let mut mut_lexers = lexers;
                                mut_lexers.insert(0, lexer);
                                Ok(mut_lexers)
                            } else {
                                Ok(lexers)
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
                // no lexer
                None => Err(SyntaxError::new_excpeted(LexerMatch::new(
                    &self.text,
                    text_index,
                    text_index + 1,
                ))),
            }
        } else {
            // is end
            Ok(vec![])
        }
    }

    pub fn matches(&self) -> Result<Vec<Lexer>, SyntaxError> {
        self.match_text(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::sql::lexer::{lexer::Lexer, pattern::LexerPattern};

    #[test]
    fn it_patterns() {
        assert!(LexerPattern::PATTERNS
            .iter()
            .any(|item| item == &LexerPattern::PATTERN_SELECT));
        assert!(LexerPattern::PATTERNS
            .iter()
            .any(|item| item == &LexerPattern::PATTERN_FROM));
        assert!(LexerPattern::PATTERNS
            .iter()
            .any(|item| item == &LexerPattern::PATTERN_ALIAS));
        assert!(LexerPattern::PATTERNS
            .iter()
            .any(|item| item == &LexerPattern::PATTERN_STAR));
        assert!(LexerPattern::PATTERNS
            .iter()
            .any(|item| item == &LexerPattern::PATTERN_COMMA));
        assert!(LexerPattern::PATTERNS
            .iter()
            .any(|item| item == &LexerPattern::PATTERN_WHITESPACE));
        assert!(LexerPattern::PATTERNS
            .iter()
            .any(|item| item == &LexerPattern::PATTERN_NAME));
    }

    #[test]
    fn it_new() {
        let pattern = LexerPattern::new("SELECT * FROM table1");
        assert_eq!(pattern.text, "SELECT * FROM table1");
    }

    #[test]
    fn it_match_pattern() {
        // Lexer::SELECT
        assert!(matches!(
            LexerPattern::new("SELECT").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::SELECT(value) if value.as_str() == "SELECT")
        ));
        assert!(matches!(
            LexerPattern::new("select").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::SELECT(value) if value.as_str() == "select")
        ));
        // Lexer::FROM
        assert!(matches!(
            LexerPattern::new("FROM").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::FROM(value) if value.as_str() == "FROM")
        ));
        assert!(matches!(
            LexerPattern::new("from").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::FROM(value) if value.as_str() == "from")
        ));
        // Lexer::AS
        assert!(matches!(
            LexerPattern::new("AS").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::ALIAS(value) if value.as_str() == "AS")
        ));
        assert!(matches!(
            LexerPattern::new("as").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::ALIAS(value) if value.as_str() == "as")
        ));
        // Lexer::STAR
        assert!(matches!(
            LexerPattern::new("*").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::STAR(value) if value.as_str() == "*")
        ));
        // Lexer::COMMA
        assert!(matches!(
            LexerPattern::new(",").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::COMMA(value) if value.as_str() == ",")
        ));
        // Lexer::WHITESPACE
        assert!(matches!(
            LexerPattern::new("' \r\n\t'").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::WHITESPACE(value) if value.as_str() == " \r\n\t")
        ));
        // Lexer::NAME
        assert!(matches!(
            LexerPattern::new("aA1_").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::NAME(value) if value.as_str() == "aA1_")
        ));
        assert!(matches!(
            LexerPattern::new("Aa1_").match_pattern(0, 0),
            Some(lexer) if matches!(&lexer, Lexer::NAME(value) if value.as_str() == "Aa1_")
        ));
        assert!(matches!(
            LexerPattern::new("_aA1").match_pattern(0, 0),
            None
        ));
        assert!(matches!(
            LexerPattern::new("1aA_").match_pattern(0, 0),
            None
        ));
    }

    #[test]
    fn it_matches() {
        // lower case
        assert!(matches!(
            LexerPattern::new("select * from table_1").matches(),
            Ok(lexers) if
                lexers.len() == 4 &&
                matches!(lexers.get(0), Some(lexer) if matches!(lexer, Lexer::SELECT(value) if value.as_str() == "select")) &&
                matches!(lexers.get(1), Some(lexer) if matches!(lexer, Lexer::STAR(value) if value.as_str() == "*")) &&
                matches!(lexers.get(2), Some(lexer) if matches!(lexer, Lexer::FROM(value) if value.as_str() == "from")) &&
                matches!(lexers.get(3), Some(lexer) if matches!(lexer, Lexer::NAME(value) if value.as_str() == "table_1"))
        ));
        // upper case
        assert!(matches!(
            LexerPattern::new("SELECT * FROM TABLE_1").matches(),
            Ok(lexers) if
                lexers.len() == 4 &&
                matches!(lexers.get(0), Some(lexer) if matches!(lexer, Lexer::SELECT(value) if value.as_str() == "SELECT")) &&
                matches!(lexers.get(1), Some(lexer) if matches!(lexer, Lexer::STAR(value) if value.as_str() == "*")) &&
                matches!(lexers.get(2), Some(lexer) if matches!(lexer, Lexer::FROM(value) if value.as_str() == "FROM")) &&
                matches!(lexers.get(3), Some(lexer) if matches!(lexer, Lexer::NAME(value) if value.as_str() == "TABLE_1"))
        ));
        // err
        assert!(matches!(
            LexerPattern::new("select * from _table1").matches(),
            Err(err) if err.cause == "expected _"
        ));
    }
}
