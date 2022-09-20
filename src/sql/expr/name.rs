use rsdb::Named;

use crate::sql::{
    err::SyntaxError,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameExpr {
    pub value: String,
}

impl Named for NameExpr {
    const NAMED: &'static str = "name expression";
}

impl LexerParser for NameExpr {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError>
    where
        Self: Sized,
    {
        match source.items.get(index) {
            Some(lexer) => match lexer {
                Lexer::NAME(value) => Ok((NameExpr::new(value.as_str()), index + 1)),
                _ => Err(SyntaxError::new_missing(lexer.value(), Self::NAMED)),
            },
            None => Err(SyntaxError::new_missing(
                LexerMatch::new_eof(&source.text),
                Self::NAMED,
            )),
        }
    }
}

impl NameExpr {
    pub fn new(value: &str) -> Self {
        NameExpr {
            value: value.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use rsdb::Named;

    use crate::sql::{
        lexer::{lexer::Lexer, mat::LexerMatch},
        parser::{LexerParser, SyntaxPattern},
    };

    use super::NameExpr;

    #[test]
    fn it_name() {
        assert_eq!(NameExpr::NAMED, "name expression");
    }

    #[test]
    fn it_new() {
        assert_eq!(NameExpr::new("4812").value, "4812");
    }

    #[test]
    fn it_parse() {
        // lexers is end
        let mut source = SyntaxPattern::new("", vec![]);
        assert!(matches!(
            NameExpr::parse(&source, 0),
            Err(err) if err.cause == "missing name expression"
        ));
        // current lexer is not a name expression
        source = SyntaxPattern::new("", vec![Lexer::SELECT(LexerMatch::new("SELECT", 0, 5))]);
        assert!(matches!(
            NameExpr::parse(&source, 0),
            Err(err) if err.cause == "missing name expression"
        ));
        // current lexer is not a name expression
        source = SyntaxPattern::new("", vec![Lexer::NAME(LexerMatch::new("8d06", 0, 4))]);
        assert!(matches!(
            NameExpr::parse(&source, 0),
            Ok((expr, index)) if expr.value == "8d06" && index == 1
        ));
    }
}
