use rsdb::Named;

use crate::sql::{
    err::SyntaxError,
    expr::{alias::AliasExpr, items::ItemsExpr},
    frag::select_item::SelectItem,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectClause {
    pub items: ItemsExpr<AliasExpr<SelectItem>>,
}

impl Named for SelectClause {
    const NAMED: &'static str = "select clause";
}

impl LexerParser for SelectClause {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError> {
        match source.items.get(index) {
            Some(lexer) => match lexer {
                Lexer::SELECT(_) => match ItemsExpr::parse(source, index + 1) {
                    Ok((items, end_index)) => match items.min_len_check(source, index + 1, 1) {
                        Some(err) => Err(err),
                        None => Ok((
                            SelectClause {
                                items: items.clone(),
                            },
                            end_index,
                        )),
                    },
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
