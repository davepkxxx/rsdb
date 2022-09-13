use rsdb::Offset;

use super::{
    err::SyntaxError,
    frag::{FromItems, NamedItemsLexerParser, SelectItems},
    lexer::Lexer,
    parser::{LexerParser, Named},
};

#[derive(Debug, Clone)]
pub struct FromClause {
    pub items: FromItems,
}

impl Named for FromClause {
    const NAME: &'static str = "from clause";
}

impl LexerParser for FromClause {
    fn parse(source: &Vec<Lexer>, mut offset: &mut Offset) -> Result<Self, SyntaxError> {
        offset.value += 1;
        match FromItems::parse(source, offset) {
            Ok(items) => Ok(FromClause { items }),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectClause {
    pub items: SelectItems,
}

impl Named for SelectClause {
    const NAME: &'static str = "select clause";
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
