use rsdb::{Named, Offset};

use crate::sql::{err::SyntaxError, expr::name::NameExpr, lexer::Lexer, parser::LexerParser};

/// Alias Expression
#[derive(Debug, Clone)]
pub struct AliasExpr<T>
where
    T: Sized,
{
    pub value: T,
    pub alias: Option<NameExpr>,
}

impl<T> Named for AliasExpr<T> {
    const NAMED: &'static str = "alias expression";
}

impl<T> AliasExpr<T> {
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


/// Parse alias expression
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
///
/// struct DemoExpr {}
///
/// impl LexerParser for DemoExpr {
///     fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError>
///     where
///         Self: Sized,
///     {
///         ...
///     }
/// }
///
/// impl AliasParser for DemoExpr {}
/// ```
pub trait AliasParser: LexerParser {
    /// Parse alias expression
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// fn run(source: &Vec<Lexer>, &mut offset: Offset) {
    ///     let expr = DemoExpr::parse_alias(&lexers, &mut offset);
    /// }
    /// ```
    fn parse_alias(source: &Vec<Lexer>, offset: &mut Offset) -> Result<AliasExpr<Self>, SyntaxError>
    where
        Self: Sized,
    {
        let value = match Self::parse(source, offset) {
            Ok(value) => value,
            Err(err) => return Err(err),
        };

        match source.get(offset.value) {
            Some(lexer) => match lexer {
                Lexer::AS(_) => match NameExpr::parse(source, offset.increment(1)) {
                    Ok(alias) => Ok(AliasExpr::new(value, alias)),
                    Err(err) => Err(err),
                },
                _ => Ok(AliasExpr::new_without_alias(value)),
            },
            None => Ok(AliasExpr::new_without_alias(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use rsdb::{Named, Offset};

    use crate::sql::{
        err::SyntaxError,
        expr::{
            alias::{AliasExpr, AliasParser},
            name::NameExpr,
        },
        lexer::Lexer,
        parser::LexerParser,
    };

    struct TestExpr {
        pub value: String,
    }

    impl LexerParser for TestExpr {
        fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError>
        where
            Self: Sized,
        {
            match source.get(offset.value) {
                Some(lexer) => {
                    offset.increment(1);
                    Ok(TestExpr {
                        value: String::from(lexer.value()),
                    })
                }
                None => Err(SyntaxError::new("4bde")),
            }
        }
    }

    impl AliasParser for TestExpr {}

    #[test]
    fn it_name() {
        assert_eq!(AliasExpr::<String>::NAMED, "alias expression");
    }

    #[test]
    fn it_new() {
        let expr = AliasExpr::new(String::from("726d"), NameExpr::new("85f5"));
        assert_eq!(expr.value, "726d");
        assert!(matches!(expr.alias, Some(name) if &name.value == "85f5"));
    }

    #[test]
    fn it_build_without_alias() {
        let expr = AliasExpr::new_without_alias(String::from("87c5"));
        assert_eq!(expr.value, "87c5");
        assert!(matches!(expr.alias, None));
    }

    #[test]
    fn it_parse_alias() {
        // parse function return an error
        let mut lexers = vec![];
        let mut offset = Offset::new(0);
        assert!(
            matches!(TestExpr::parse_alias(&lexers, &mut offset), Err(err) if err.msg == "SyntaxError: 4bde")
        );
        assert_eq!(offset.value, 0);
        // lexers is end after parse function call
        lexers = vec![Lexer::NAME(String::from("9a42"))];
        offset.value = 0;
        assert!(
            matches!(TestExpr::parse_alias(&lexers, &mut offset), Ok(expr) if &expr.value.value == "9a42" && matches!(expr.alias, None))
        );
        assert_eq!(offset.value, 1);
        // no AS in lexers
        lexers = vec![Lexer::NAME(String::from("b579"))];
        offset.value = 0;
        assert!(
            matches!(TestExpr::parse_alias(&lexers, &mut offset), Ok(expr) if &expr.value.value == "b579" && matches!(expr.alias, None))
        );
        assert_eq!(offset.value, 1);
        // lexers ends with AS
        lexers = vec![
            Lexer::NAME(String::from("43f4")),
            Lexer::AS(String::from("AS")),
        ];
        offset.value = 0;
        assert!(
            matches!(TestExpr::parse_alias(&lexers, &mut offset), Err(err) if err.msg == "SyntaxError: missing name expression")
        );
        assert_eq!(offset.value, 2);
        // no name expression follows AS
        lexers = vec![
            Lexer::NAME(String::from("7888")),
            Lexer::AS(String::from("AS")),
            Lexer::SELECT(String::from("select")),
        ];
        offset.value = 0;
        assert!(
            matches!(TestExpr::parse_alias(&lexers, &mut offset), Err(err) if err.msg == "SyntaxError: missing name expression")
        );
        assert_eq!(offset.value, 2);
        // contain alias expression
        lexers = vec![
            Lexer::NAME(String::from("f9b8")),
            Lexer::AS(String::from("AS")),
            Lexer::NAME(String::from("0a54")),
        ];
        offset.value = 0;
        assert!(matches!(
            TestExpr::parse_alias(&lexers,
            &mut offset), Ok(expr) if &expr.value.value == "f9b8" && matches!(&expr.alias, Some(name_expr) if name_expr.value == "0a54")
        ));
        assert_eq!(offset.value, 3);
    }
}
