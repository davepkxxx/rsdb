use rsdb::{Named, Offset};

use crate::sql::{err::SyntaxError, lexer::Lexer};

pub trait ItemsParser: Named {
    type Item;

    fn new(items: Vec<Self::Item>) -> Self;

    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError>
    where
        Self: Sized,
    {
        match Self::parse_items(source, offset) {
            Ok(items) => {
                if items.len() > 0 {
                    Ok(Self::new(items))
                } else {
                    match source.get(offset.value) {
                        Some(lexer) => Err(SyntaxError::new_excpeted(lexer.value())),
                        None => Err(SyntaxError::new_missing(Self::NAMED)),
                    }
                }
            }
            Err(err) => Err(err),
        }
    }

    fn parse_items(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Vec<Self::Item>, SyntaxError>
    where
        Self: Sized,
    {
        let mut items: Vec<Self::Item> = vec![];

        if matches!(source.get(offset.value), Some(_)) {
            match Self::parse_item(source, offset) {
                Ok(item) => items.push(item),
                Err(err) => return Err(err),
            }

            if let Some(lexer) = source.get(offset.value) {
                if matches!(lexer, Lexer::COMMA(_)) {
                    match Self::parse_items(source, offset.increment(1)) {
                        Ok(more_items) => {
                            for item in more_items {
                                items.push(item)
                            }
                        }
                        Err(err) => return Err(err),
                    }
                }
            }
        }

        Ok(items)
    }

    fn parse_item(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self::Item, SyntaxError>;
}
