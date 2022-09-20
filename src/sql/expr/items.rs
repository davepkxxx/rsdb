use rsdb::Named;

use crate::sql::{
    err::SyntaxError,
    lexer::{lexer::Lexer, mat::LexerMatch},
    parser::{LexerParser, SyntaxPattern},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemsExpr<T>
where
    T: Sized,
{
    pub items: Vec<T>,
}

impl<T> Named for ItemsExpr<T>
where
    T: Sized,
{
    const NAMED: &'static str = "items";
}

impl<T> LexerParser for ItemsExpr<T>
where
    T: Sized + LexerParser,
{
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError>
    where
        Self: Sized,
    {
        // get item
        match source.items.get(index) {
            // has item & parse item
            Some(_) => match T::parse(source, index) {
                // parse item ok & get lexer
                Ok((item, item_end_index)) => match source.items.get(item_end_index) {
                    // has lexer & check lexer
                    Some(lexer) => match lexer {
                        // lexer is comma & parse next items
                        Lexer::COMMA(_) => match Self::parse(source, item_end_index + 1) {
                            // parse next items ok & check next items's length
                            Ok((next_items, next_end_index)) => {
                                match next_items.min_len_check(source, item_end_index + 1, 1) {
                                    // next items is empty
                                    Some(err) => Err(err),
                                    // next items is not empty
                                    None => {
                                        let mut mut_items = next_items;
                                        mut_items.items.insert(0, item);
                                        Ok((mut_items, next_end_index))
                                    }
                                }
                            }
                            Err(err) => Err(err),
                        },
                        // lexer is not comma
                        _ => Ok((Self::new(vec![item]), item_end_index)),
                    },
                    // lexers is end, no comma
                    None => Ok((Self::new(vec![item]), item_end_index)),
                },
                // parse item err
                Err(err) => Err(err),
            },
            // lexers is end, no item
            None => Ok((Self::new(vec![]), index)),
        }
    }
}

impl<T> ItemsExpr<T>
where
    T: Sized,
{
    pub fn new(items: Vec<T>) -> Self {
        ItemsExpr { items }
    }

    pub fn min_len_check(
        &self,
        source: &SyntaxPattern,
        index: usize,
        min: usize,
    ) -> Option<SyntaxError> {
        if self.items.len() < min {
            match source.items.get(index) {
                Some(lexer) => Some(SyntaxError::new_missing(lexer.value(), Self::NAMED)),
                None => Some(SyntaxError::new_missing(
                    LexerMatch::new_eof(&source.text),
                    Self::NAMED,
                )),
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use rsdb::Named;

    use crate::sql::{
        expr::name::NameExpr,
        lexer::{lexer::Lexer, mat::LexerMatch},
        parser::{LexerParser, SyntaxPattern},
    };

    use super::ItemsExpr;

    #[test]
    fn it_name() {
        assert_eq!(ItemsExpr::<NameExpr>::NAMED, "items");
    }

    #[test]
    fn it_new() {
        assert_eq!(
            ItemsExpr::<NameExpr>::new(vec![NameExpr::new("d208")]).items,
            [NameExpr::new("d208")]
        );
    }

    #[test]
    fn it_len_check() {
        let source = SyntaxPattern::new("", vec![]);
        let expr = ItemsExpr::new(vec![NameExpr::new("")]);
        // len greater than min
        assert!(matches!(expr.min_len_check(&source, 0, 0), None));
        // len equal min
        assert!(matches!(expr.min_len_check(&source, 0, 1), None));
        // len less than min
        assert!(matches!(
            expr.min_len_check(&source, 0, 2),
            Some(err) if err.cause == "missing items"
        ));
    }

    #[test]
    fn it_parse() {
        // no item
        let mut source = SyntaxPattern::new("", vec![]);
        assert!(matches!(
            ItemsExpr::<NameExpr>::parse(&source, 0),
            Ok((expr, index)) if expr.items == vec![] && index == 0
        ));
        // one items
        source = SyntaxPattern::new(
            "cee1",
            vec![Lexer::NAME(LexerMatch::new_full_match("cee1"))],
        );
        assert!(matches!(
            ItemsExpr::<NameExpr>::parse(&source, 0),
            Ok((expr, index)) if expr.items == vec![NameExpr::new("cee1")] && index == 1
        ));
        // two items
        source = SyntaxPattern::new(
            "57f3, fe80",
            vec![
                Lexer::NAME(LexerMatch::new("57f3, fe80", 0, 4)),
                Lexer::COMMA(LexerMatch::new("57f3, fe80", 4, 5)),
                Lexer::NAME(LexerMatch::new("57f3, fe80", 6, 10)),
            ],
        );
        assert!(matches!(
            ItemsExpr::<NameExpr>::parse(&source, 0),
            Ok((expr, index)) if expr.items == vec![NameExpr::new("57f3"), NameExpr::new("fe80")] && index == 3
        ));
        // no item after the comma
        source = SyntaxPattern::new(
            "410a,",
            vec![
                Lexer::NAME(LexerMatch::new("410a,", 0, 4)),
                Lexer::COMMA(LexerMatch::new("410a,", 4, 5)),
            ],
        );
        assert!(matches!(
            ItemsExpr::<NameExpr>::parse(&source, 0),
            Err(err) if err.cause == "missing items"
        ));
    }
}
