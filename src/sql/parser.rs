use super::{
    err::SyntaxError,
    lexer::{lexer::Lexer, pattern::LexerPattern},
    stmt::stmt::Stmt,
};

pub trait LexerParser {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxPattern {
    pub text: String,
    pub items: Vec<Lexer>,
}

impl SyntaxPattern {
    pub fn new(text: &str, items: Vec<Lexer>) -> Self {
        SyntaxPattern {
            text: text.to_owned(),
            items,
        }
    }

    pub fn first(&self) -> Option<&Lexer> {
        self.items.get(0)
    }
}

pub fn parse_stmt(sql: &str) -> Result<Stmt, SyntaxError> {
    match LexerPattern::new(sql).matches() {
        Ok(lexers) => match Stmt::parse(&SyntaxPattern::new(sql, lexers), 0) {
            Ok((stmt, _)) => Ok(stmt),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
