use rsdb::Offset;

use super::{err::SyntaxError, lexer::Lexer, stmt::Stmt};

pub trait Named {
    const NAME: &'static str;
}

pub trait NamedInstance {
    fn name(&self) -> &'static str;
}

pub trait LexerParser {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError>
    where
        Self: Sized;
}

pub fn parse(sql: &str) -> Result<Stmt, SyntaxError> {
    match Lexer::parse(sql) {
        Ok(lexers) => Stmt::parse(&lexers, &mut Offset::new(0)),
        Err(err) => Err(err),
    }
}
