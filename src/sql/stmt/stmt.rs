use rsdb::{Named, Offset};

use crate::sql::{err::SyntaxError, lexer::Lexer, parser::LexerParser};

use super::select::SelectStmt;

#[derive(Debug)]
pub enum Stmt {
    SELECT(SelectStmt),
}

impl Named for Stmt {
    const NAMED: &'static str = "statement";
}

impl LexerParser for Stmt {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        match source.get(offset.value) {
            Some(lexer) => match lexer {
                Lexer::SELECT(_) => match SelectStmt::parse(source, offset) {
                    Ok(stmt) => Ok(Stmt::SELECT(stmt)),
                    Err(err) => Err(err),
                },
                _ => Err(SyntaxError::new_excpeted(lexer.value())),
            },
            None => Err(SyntaxError::new_missing(Self::NAMED)),
        }
    }
}
