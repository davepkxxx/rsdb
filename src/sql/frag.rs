use rsdb::Offset;

use super::{
    err::SyntaxError,
    lexer::Lexer,
    parser::{LexerParser, Named},
};

pub trait NamedItemsLexerParser: Named {
    type Item;

    fn new(items: Vec<Self::Item>) -> Self;

    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError>
    where
        Self: Sized,
    {
        let mut items: Vec<Self::Item> = vec![];

        loop {
            match source.get(offset.value) {
                Some(_) => match Self::parse_item(source, offset) {
                    Ok(item) => items.push(item),
                    Err(err) => return Err(err),
                },
                None => break,
            }

            match source.get(offset.value) {
                Some(lexer) => {
                    if matches!(lexer, Lexer::COMMA(_)) {
                        offset.value += 1;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        if items.len() > 0 {
            Ok(Self::new(items))
        } else {
            match source.get(offset.value) {
                Some(lexer) => Err(SyntaxError::build_excpeted_err(lexer.value())),
                None => Err(SyntaxError::build_miss_err(Self::NAME)),
            }
        }
    }

    fn parse_item(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self::Item, SyntaxError>;
}

#[derive(Debug, Clone)]
pub struct FromItems {
    pub items: Vec<FromItem>,
}

impl Named for FromItems {
    const NAME: &'static str = "select items";
}

impl NamedItemsLexerParser for FromItems {
    type Item = FromItem;

    fn new(items: Vec<Self::Item>) -> Self {
        FromItems { items }
    }

    fn parse_item(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self::Item, SyntaxError> {
        FromItem::parse(source, offset)
    }
}

#[derive(Debug, Clone)]
pub enum FromItem {
    ID(String),
}

impl Named for FromItem {
    const NAME: &'static str = "from item";
}

impl LexerParser for FromItem {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        match source.get(offset.value).unwrap() {
            Lexer::ID(value) => {
                offset.value += 1;
                Ok(FromItem::ID(value.clone()))
            }
            _ => Err(SyntaxError::build_miss_err(SelectItem::NAME)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectItems {
    pub items: Vec<SelectItem>,
}

impl Named for SelectItems {
    const NAME: &'static str = "select items";
}

impl NamedItemsLexerParser for SelectItems {
    type Item = SelectItem;

    fn new(items: Vec<Self::Item>) -> Self {
        SelectItems { items }
    }

    fn parse_item(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self::Item, SyntaxError> {
        SelectItem::parse(source, offset)
    }
}

#[derive(Debug, Clone)]
pub enum SelectItem {
    ID(String),
    STAR,
}

impl Named for SelectItem {
    const NAME: &'static str = "select item";
}

impl LexerParser for SelectItem {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        match source.get(offset.value).unwrap() {
            Lexer::ID(value) => {
                offset.value += 1;
                Ok(SelectItem::ID(value.clone()))
            }
            Lexer::STAR(_) => {
                offset.value += 1;
                Ok(SelectItem::STAR)
            }
            _ => Err(SyntaxError::build_miss_err(SelectItem::NAME)),
        }
    }
}
