use rsdb::{Named, NamedEnum};

use crate::sql::{
    err::SyntaxError,
    expr::name::NameExpr,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FromItem {
    NAME(NameExpr),
}

impl Named for FromItem {
    const NAMED: &'static str = "from item";
}

impl NamedEnum for FromItem {
    fn name(&self) -> &'static str {
        match self {
            Self::NAME(_) => NameExpr::NAMED,
        }
    }
}

impl LexerParser for FromItem {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError> {
        match source.items.get(index) {
            Some(lexer) => match lexer {
                Lexer::NAME(_) => match NameExpr::parse(source, index) {
                    Ok((expr, end_index)) => Ok((FromItem::NAME(expr), end_index)),
                    Err(err) => Err(err),
                },
                _ => Err(SyntaxError::new_missing(lexer.value(), FromItem::NAMED)),
            },
            None => Err(SyntaxError::new_missing(
                LexerMatch::new_eof(&source.text),
                FromItem::NAMED,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use rsdb::{Named, NamedEnum};

    use crate::sql::{
        expr::name::NameExpr,
        lexer::{lexer::Lexer, mat::LexerMatch},
        parser::{LexerParser, SyntaxPattern},
    };

    use super::FromItem;

    #[test]
    fn it_name() {
        assert_eq!(FromItem::NAMED, "from item");
        assert_eq!(FromItem::NAME(NameExpr::new("")).name(), "name expression");
    }

    #[test]
    fn it_parse() {
        // lexers is end
        let mut source = SyntaxPattern::new("", vec![]);
        assert!(matches!(
            FromItem::parse(&source, 0),
            Err(err) if err.cause == "missing from item"
        ));
        // no from item in lexers
        source = SyntaxPattern::new(
            "SELECT",
            vec![Lexer::SELECT(LexerMatch::new_full_match("SELECT"))],
        );
        assert!(matches!(
            FromItem::parse(&source, 0),
            Err(err) if err.cause == "missing from item"
        ));
        // contain from item -> name expression
        source = SyntaxPattern::new(
            "3210",
            vec![Lexer::NAME(LexerMatch::new_full_match("3210"))],
        );
        assert!(matches!(
            FromItem::parse(&source, 0),
            Ok((item,index)) if matches!(&item, FromItem::NAME(expr) if expr.value == "3210") && index == 1
        ));
    }
}
