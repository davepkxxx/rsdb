use rsdb::{Named, Offset};

use crate::sql::{
    err::SyntaxError,
    frag::{items::ItemsParser, select_items::SelectItems},
    lexer::Lexer,
    parser::LexerParser,
};

#[derive(Debug, Clone)]
pub struct SelectClause {
    pub items: SelectItems,
}

impl Named for SelectClause {
    const NAMED: &'static str = "select clause";
}

impl LexerParser for SelectClause {
    fn parse(source: &Vec<Lexer>, mut offset: &mut Offset) -> Result<Self, SyntaxError> {
        offset.value += 1;
        match SelectItems::parse(source, offset) {
            Ok(items) => Ok(SelectClause { items }),
            Err(err) => Err(err),
        }
    }
}
