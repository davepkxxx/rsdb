use rsdb::Offset;

use super::{
    clause::{FromClause, SelectClause},
    err::SyntaxError,
    lexer::Lexer,
    parser::{LexerParser, Named},
};

#[derive(Debug)]
pub enum Stmt {
    SELECT(SelectStmt),
}

impl Named for Stmt {
    const NAME: &'static str = "statement";
}

impl LexerParser for Stmt {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        match source.get(offset.value) {
            Some(lexer) => match lexer {
                Lexer::SELECT(_) => match SelectStmt::parse(source, offset) {
                    Ok(stmt) => Ok(Stmt::SELECT(stmt)),
                    Err(err) => Err(err),
                },
                _ => Err(SyntaxError::build_excpeted_err(lexer.value())),
            },
            None => Err(SyntaxError::build_miss_err(Self::NAME)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectStmt {
    pub select_clause: SelectClause,
    pub from_clause: FromClause,
}

impl Named for SelectStmt {
    const NAME: &'static str = "select statement";
}

impl LexerParser for SelectStmt {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        let select_clause = match SelectClause::parse(source, offset) {
            Ok(clause) => clause,
            Err(err) => return Err(err),
        };

        let from_clause = match source.get(offset.value) {
            Some(lexer) => match lexer {
                Lexer::FROM(_) => match FromClause::parse(source, offset) {
                    Ok(clause) => clause,
                    Err(err) => return Err(err),
                },
                _ => {
                    println!("excepted {}", lexer.value());
                    return Err(SyntaxError::build_miss_err(FromClause::NAME))
                },
            },
            None => return Err(SyntaxError::build_miss_err(FromClause::NAME)),
        };

        Ok(SelectStmt {
            select_clause,
            from_clause,
        })
    }
}
