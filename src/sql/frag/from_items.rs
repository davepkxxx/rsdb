use rsdb::{Named, Offset};

use crate::sql::{
    err::SyntaxError,
    expr::alias::{AliasExpr, AliasParser},
    lexer::Lexer,
};

use super::{from_item::FromItem, items::ItemsParser};

#[derive(Debug, Clone)]
pub struct FromItems {
    pub items: Vec<AliasExpr<FromItem>>,
}

impl Named for FromItems {
    const NAMED: &'static str = "select items";
}

impl ItemsParser for FromItems {
    type Item = AliasExpr<FromItem>;

    fn new(items: Vec<Self::Item>) -> Self {
        FromItems { items }
    }

    fn parse_item(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self::Item, SyntaxError> {
        FromItem::parse_alias(source, offset)
    }
}
