use rsdb::{Is, NamedEnum};

use super::mat::LexerMatch;

/// The SQL lexer
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lexer {
    SELECT(LexerMatch),
    FROM(LexerMatch),
    ALIAS(LexerMatch),
    STAR(LexerMatch),
    COMMA(LexerMatch),
    WHITESPACE(LexerMatch),
    NAME(LexerMatch),
}

impl NamedEnum for Lexer {
    fn name(&self) -> &'static str {
        match self {
            Self::SELECT(_) => "SELECT",
            Self::FROM(_) => "FROM",
            Self::ALIAS(_) => "AS",
            Self::STAR(_) => "STAR",
            Self::COMMA(_) => "COMMA",
            Self::WHITESPACE(_) => "WHITESPACE",
            Self::NAME(_) => "NAME",
        }
    }
}

impl Is for Lexer {
    fn is(&self, other: &Self) -> bool {
        match self {
            Self::SELECT(_) => matches!(other, Self::SELECT(_)),
            Self::FROM(_) => matches!(other, Self::FROM(_)),
            Self::ALIAS(_) => matches!(other, Self::ALIAS(_)),
            Self::STAR(_) => matches!(other, Self::STAR(_)),
            Self::COMMA(_) => matches!(other, Self::COMMA(_)),
            Self::WHITESPACE(_) => matches!(other, Self::WHITESPACE(_)),
            Self::NAME(_) => matches!(other, Self::NAME(_)),
        }
    }
}

impl Lexer {
    pub fn is_value_ignores(&self) -> bool {
        matches!(
            self,
            Self::SELECT(_) | Self::FROM(_) | Self::STAR(_) | Self::COMMA(_) | Self::WHITESPACE(_)
        )
    }

    pub fn is_clause(&self) -> bool {
        matches!(self, Self::SELECT(_) | Self::FROM(_))
    }

    pub fn value(&self) -> LexerMatch {
        match self {
            Self::SELECT(value) => value,
            Self::FROM(value) => value,
            Self::ALIAS(value) => value,
            Self::STAR(value) => value,
            Self::COMMA(value) => value,
            Self::WHITESPACE(value) => value,
            Self::NAME(value) => value,
        }
        .clone()
    }
}

#[cfg(test)]
mod tests {
    use rsdb::NamedEnum;

    use crate::sql::lexer::mat::LexerMatch;

    use super::Lexer;

    fn new_lexer(name: &str, value: &str) -> Lexer {
        match name {
            "SELECT" => Lexer::SELECT(LexerMatch::new_full_match(value)),
            "FROM" => Lexer::FROM(LexerMatch::new_full_match(value)),
            "AS" => Lexer::ALIAS(LexerMatch::new_full_match(value)),
            "STAR" => Lexer::STAR(LexerMatch::new_full_match(value)),
            "COMMA" => Lexer::COMMA(LexerMatch::new_full_match(value)),
            "WHITESPACE" => Lexer::WHITESPACE(LexerMatch::new_full_match(value)),
            "NAME" => Lexer::NAME(LexerMatch::new_full_match(value)),
            _ => panic!("err lexer"),
        }
    }

    #[test]
    fn it_name() {
        assert_eq!(new_lexer("SELECT", "").name(), "SELECT");
        assert_eq!(new_lexer("FROM", "").name(), "FROM");
        assert_eq!(new_lexer("AS", "").name(), "AS");
        assert_eq!(new_lexer("STAR", "").name(), "STAR");
        assert_eq!(new_lexer("COMMA", "").name(), "COMMA");
        assert_eq!(new_lexer("WHITESPACE", "").name(), "WHITESPACE");
        assert_eq!(new_lexer("NAME", "").name(), "NAME");
    }

    #[test]
    fn it_value() {
        assert_eq!(new_lexer("SELECT", "cc45").value().as_str(), "cc45");
        assert_eq!(new_lexer("FROM", "d733").value().as_str(), "d733");
        assert_eq!(new_lexer("AS", "46c6").value().as_str(), "46c6");
        assert_eq!(new_lexer("STAR", "debb").value().as_str(), "debb");
        assert_eq!(new_lexer("COMMA", "41ce").value().as_str(), "41ce");
        assert_eq!(new_lexer("WHITESPACE", "b734").value().as_str(), "b734");
        assert_eq!(new_lexer("NAME", "ee4f").value().as_str(), "ee4f");
    }
}
