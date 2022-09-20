use rsdb::Named;

use crate::sql::{
    clause::{from::FromClause, select::SelectClause},
    err::SyntaxError,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectStmt {
    pub select_clause: SelectClause,
    pub from_clause: FromClause,
}

impl Named for SelectStmt {
    const NAMED: &'static str = "select statement";
}

impl LexerParser for SelectStmt {
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError> {
        let (select_clause, select_end_index) = match SelectClause::parse(source, index) {
            Ok(clause) => clause,
            Err(err) => return Err(err),
        };

        let (from_clause, from_end_index) = match source.items.get(select_end_index) {
            Some(lexer) => match lexer {
                Lexer::FROM(_) => match FromClause::parse(source, select_end_index) {
                    Ok(clause) => clause,
                    Err(err) => return Err(err),
                },
                _ => {
                    return Err(SyntaxError::new_missing(lexer.value(), FromClause::NAMED));
                }
            },
            None => {
                return Err(SyntaxError::new_missing(
                    LexerMatch::new_eof(&source.text),
                    FromClause::NAMED,
                ))
            }
        };

        Ok((
            SelectStmt {
                select_clause,
                from_clause,
            },
            from_end_index,
        ))
    }
}
