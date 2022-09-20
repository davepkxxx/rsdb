use rsdb::Named;

use crate::sql::{
    err::SyntaxError,
    expr::{alias::AliasExpr, items::ItemsExpr},
    frag::from_item::FromItem,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromClause {
    pub items: ItemsExpr<AliasExpr<FromItem>>,
}

impl Named for FromClause {
    const NAMED: &'static str = "from clause";
}

impl LexerParser for FromClause {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError> {
        match source.items.get(index) {
            Some(lexer) => match lexer {
                Lexer::FROM(_) => match ItemsExpr::parse(source, index + 1) {
                    Ok((items, end_index)) => Ok(((FromClause { items }), end_index)),
                    Err(err) => Err(err),
                },
                _ => Err(SyntaxError::new_missing(lexer.value(), Self::NAMED)),
            },
            None => Err(SyntaxError::new_missing(
                LexerMatch::new_eof(&source.text),
                Self::NAMED,
            )),
        }
    }
}
