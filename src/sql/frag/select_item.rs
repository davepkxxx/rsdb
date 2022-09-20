use rsdb::{Named, NamedEnum};

use crate::sql::{
    err::SyntaxError,
    expr::name::NameExpr,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectItem {
    NAME(NameExpr),
    STAR,
}

impl Named for SelectItem {
    const NAMED: &'static str = "select item";
}

impl NamedEnum for SelectItem {
    fn name(&self) -> &'static str {
        match self {
            Self::NAME(_) => NameExpr::NAMED,
            Self::STAR => "STAR",
        }
    }
}

impl LexerParser for SelectItem {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError> {
        match source.items.get(index) {
            Some(lexer) => match lexer {
                Lexer::NAME(_) => match NameExpr::parse(source, index) {
                    Ok((expr, end_index)) => Ok((SelectItem::NAME(expr), end_index)),
                    Err(err) => Err(err),
                },
                Lexer::STAR(_) => Ok((SelectItem::STAR, index + 1)),
                _ => Err(SyntaxError::new_missing(lexer.value(), SelectItem::NAMED)),
            },
            None => Err(SyntaxError::new_missing(
                LexerMatch::new_eof(&source.text),
                SelectItem::NAMED,
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

    use super::SelectItem;

    #[test]
    fn it_name() {
        assert_eq!(SelectItem::NAMED, "select item");
        assert_eq!(
            SelectItem::NAME(NameExpr::new("")).name(),
            "name expression"
        );
        assert_eq!(SelectItem::STAR.name(), "STAR");
    }

    #[test]
    fn it_parse() {
        // lexers is end
        let mut source = SyntaxPattern::new("", vec![]);
        assert!(matches!(
            SelectItem::parse(&source, 0),
            Err(err) if err.cause == "missing select item"
        ));
        // no from item in lexers
        source = SyntaxPattern::new(
            "SELECT",
            vec![Lexer::SELECT(LexerMatch::new_full_match("SELECT"))],
        );
        assert!(matches!(
            SelectItem::parse(&source, 0),
            Err(err) if err.cause == "missing select item"
        ));
        // contain from item -> name expression
        source = SyntaxPattern::new(
            "4dfa",
            vec![Lexer::NAME(LexerMatch::new_full_match("4dfa"))],
        );
        assert!(matches!(
            SelectItem::parse(&source, 0),
            Ok((item, index)) if matches!(&item, SelectItem::NAME(expr) if expr.value == "4dfa" && index == 1
        )));
        // contain from item -> name expression
        source = SyntaxPattern::new("*", vec![Lexer::STAR(LexerMatch::new_full_match("*"))]);
        assert!(matches!(
            SelectItem::parse(&source, 0),
            Ok((item, index)) if matches!(item, SelectItem::STAR) && index == 1
        ));
    }
}
