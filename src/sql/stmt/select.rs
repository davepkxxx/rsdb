use rsdb::{Named, Offset};

use crate::sql::{
    clause::{from::FromClause, select::SelectClause},
    err::SyntaxError,
    lexer::Lexer,
    parser::LexerParser,
};

#[derive(Debug, Clone)]
pub struct SelectStmt {
    pub select_clause: SelectClause,
    pub from_clause: FromClause,
}

impl Named for SelectStmt {
    const NAMED: &'static str = "select statement";
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
                    return Err(SyntaxError::new_missing(FromClause::NAMED));
                }
            },
            None => return Err(SyntaxError::new_missing(FromClause::NAMED)),
        };

        Ok(SelectStmt {
            select_clause,
            from_clause,
        })
    }
}