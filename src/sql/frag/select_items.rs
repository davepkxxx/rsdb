use rsdb::{Named, Offset};

use crate::sql::{
    err::SyntaxError,
    expr::alias::{AliasExpr, AliasParser},
    lexer::Lexer,
};

use super::{items::ItemsParser, select_item::SelectItem};

#[derive(Debug, Clone)]
pub struct SelectItems {
    pub items: Vec<AliasExpr<SelectItem>>,
}

impl Named for SelectItems {
    const NAMED: &'static str = "select items";
}

impl ItemsParser for SelectItems {
    type Item = AliasExpr<SelectItem>;

    fn new(items: Vec<Self::Item>) -> Self {
        SelectItems { items }
    }

    fn parse_item(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self::Item, SyntaxError> {
        SelectItem::parse_alias(source, offset)
    }
}
