use rsdb::{Named, Offset};

use crate::sql::{
    err::SyntaxError,
    frag::{from_items::FromItems, items::ItemsParser},
    lexer::Lexer,
    parser::LexerParser,
};

#[derive(Debug, Clone)]
pub struct FromClause {
    pub items: FromItems,
}

impl Named for FromClause {
    const NAMED: &'static str = "from clause";
}

impl LexerParser for FromClause {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        offset.value += 1;
        match FromItems::parse(source, offset) {
            Ok(items) => Ok(FromClause { items }),
            Err(err) => Err(err),
        }
    }
}
