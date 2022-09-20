use rsdb::Named;

use crate::sql::{
    err::SyntaxError,
    expr::name::NameExpr,
    lexer::lexer::Lexer,
    parser::{LexerParser, SyntaxPattern},
};

/// Alias Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasExpr<T>
where
    T: Sized + LexerParser,
{
    pub value: T,
    pub alias: Option<NameExpr>,
}

impl<T> Named for AliasExpr<T>
where
    T: Sized + LexerParser,
{
    const NAMED: &'static str = "alias expression";
}

impl<T> LexerParser for AliasExpr<T>
where
    T: Sized + LexerParser,
{
    fn parse(source: &SyntaxPattern, index: usize) -> Result<(Self, usize), SyntaxError>
    where
        Self: Sized,
    {
        match T::parse(source, index) {
            Ok((value, value_end_index)) => match source.items.get(value_end_index) {
                Some(lexer) => match lexer {
                    Lexer::ALIAS(_) => match NameExpr::parse(source, value_end_index + 1) {
                        Ok((alias, alias_end_index)) => {
                            Ok((AliasExpr::new(value, alias), alias_end_index))
                        }
                        Err(err) => Err(err),
                    },
                    _ => Ok((AliasExpr::new_without_alias(value), value_end_index)),
                },
                None => Ok((AliasExpr::new_without_alias(value), value_end_index)),
            },
            Err(err) => Err(err),
        }
    }
}

impl<T> AliasExpr<T>
where
    T: Sized + LexerParser,
{
    /// Creates a new alias expr
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let expr = AliasExpr::new(FromItem::NAME(NameExpr::new("table_1")), NameExpr::new("t1"));
    /// ```
    pub fn new(value: T, alias: NameExpr) -> Self {
        AliasExpr {
            value,
            alias: Some(alias),
        }
    }

    /// Creates a new alias expr with out alias
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// let expr = AliasExpr::new_without_alias(FromItem::NAME(NameExpr::new("table_1")));
    /// ```
    pub fn new_without_alias(value: T) -> Self {
        AliasExpr { value, alias: None }
    }
}

#[cfg(test)]
mod tests {
    use rsdb::Named;

    use crate::sql::{
        expr::{alias::AliasExpr, name::NameExpr},
        lexer::{lexer::Lexer, mat::LexerMatch},
        parser::{LexerParser, SyntaxPattern},
    };

    #[test]
    fn it_name() {
        assert_eq!(AliasExpr::<NameExpr>::NAMED, "alias expression");
    }

    #[test]
    fn it_new() {
        let expr = AliasExpr::new(NameExpr::new("726d"), NameExpr::new("85f5"));
        assert_eq!(expr.value.value, "726d");
        assert!(matches!(expr.alias, Some(name) if name.value == "85f5"));
    }

    #[test]
    fn it_build_without_alias() {
        let expr = AliasExpr::new_without_alias(NameExpr::new("87c5"));
        assert_eq!(expr.value.value, "87c5");
        assert!(matches!(expr.alias, None));
    }

    #[test]
    fn it_parse() {
        // parse function return an error
        let mut source = SyntaxPattern::new(
            "SELECT",
            vec![Lexer::SELECT(LexerMatch::new_full_match("SELECT"))],
        );
        assert!(matches!(
            AliasExpr::<NameExpr>::parse(&source, 0),
            Err(err) if err.cause == "missing name expression"
        ));
        // lexers is end after parse function call
        source = SyntaxPattern::new(
            "9a42",
            vec![Lexer::NAME(LexerMatch::new_full_match("9a42"))],
        );
        assert!(matches!(
            AliasExpr::<NameExpr>::parse(&source, 0),
            Ok((expr, index)) if expr.value.value == "9a42" && matches!(expr.alias, None) && index == 1
        ));
        // no AS in lexers
        source = SyntaxPattern::new(
            "b579",
            vec![Lexer::NAME(LexerMatch::new_full_match("b579"))],
        );
        assert!(matches!(
            AliasExpr::<NameExpr>::parse(&source, 0),
            Ok((expr, index)) if expr.value.value == "b579" && matches!(expr.alias, None) && index == 1
        ));
        // lexers ends with AS
        source = SyntaxPattern::new(
            "43f4 AS",
            vec![
                Lexer::NAME(LexerMatch::new("43f4 AS", 0, 4)),
                Lexer::ALIAS(LexerMatch::new("43f4 AS", 5, 7)),
            ],
        );
        assert!(matches!(
            AliasExpr::<NameExpr>::parse(&source, 0),
            Err(err) if err.cause == "missing name expression"
        ));
        // no name expression follows AS
        source = SyntaxPattern::new(
            "7888 AS SELECT",
            vec![
                Lexer::NAME(LexerMatch::new("7888 AS SELECT", 0, 4)),
                Lexer::ALIAS(LexerMatch::new("7888 AS SELECT", 5, 7)),
                Lexer::SELECT(LexerMatch::new("7888 AS SELECT", 8, 14)),
            ],
        );
        assert!(matches!(
            AliasExpr::<NameExpr>::parse(&source, 0),
            Err(err) if err.cause == "missing name expression"
        ));
        // contain alias expression
        source = SyntaxPattern::new(
            "f9b8 AS 0a54",
            vec![
                Lexer::NAME(LexerMatch::new("f9b8 AS 0a54", 0, 4)),
                Lexer::ALIAS(LexerMatch::new("f9b8 AS 0a54", 5, 7)),
                Lexer::NAME(LexerMatch::new("f9b8 AS 0a54", 8, 12)),
            ],
        );
        assert!(matches!(
            AliasExpr::<NameExpr>::parse(&source, 0),
            Ok((expr, index)) if expr.value.value == "f9b8" && matches!(&expr.alias, Some(name_expr) if name_expr.value == "0a54") && index == 3
        ));
    }
}
