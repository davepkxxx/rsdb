use rsdb::Named;

use crate::sql::{
    err::SyntaxError,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

use super::select::SelectStmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    SELECT(SelectStmt),
}

impl Named for Stmt {
    const NAMED: &'static str = "statement";
}

impl LexerParser for Stmt {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError> {
        match source.items.get(index) {
            Some(lexer) => match lexer {
                Lexer::SELECT(_) => match SelectStmt::parse(source, index) {
                    Ok((stmt, end_index)) => Ok((Stmt::SELECT(stmt), end_index)),
                    Err(err) => Err(err),
                },
                _ => Err(SyntaxError::new_excpeted(lexer.value())),
            },
            None => Err(SyntaxError::new_missing(
                LexerMatch::new_eof(&source.text),
                Self::NAMED,
            )),
        }
    }
}
