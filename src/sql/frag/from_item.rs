use rsdb::{Named, NamedEnum, Offset};

use crate::sql::{
    err::SyntaxError,
    expr::{alias::AliasParser, name::NameExpr},
    lexer::Lexer,
    parser::LexerParser,
};

#[derive(Debug, Clone)]
pub enum FromItem {
    NAME(NameExpr),
}

impl Named for FromItem {
    const NAMED: &'static str = "from item";
}

impl NamedEnum for FromItem {
    fn name(&self) -> &'_ str {
        match self {
            Self::NAME(_) => NameExpr::NAMED,
        }
    }
}

impl LexerParser for FromItem {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        match source.get(offset.value) {
            Some(lexer) => match lexer {
                Lexer::NAME(_) => Ok(FromItem::NAME(NameExpr::parse(source, offset).unwrap())),
                _ => Err(SyntaxError::new_missing(FromItem::NAMED)),
            },
            None => Err(SyntaxError::new_missing(FromItem::NAMED)),
        }
    }
}

impl AliasParser for FromItem {}

#[cfg(test)]
mod tests {
    use rsdb::{Named, NamedEnum, Offset};

    use crate::sql::{expr::name::NameExpr, lexer::Lexer, parser::LexerParser};

    use super::FromItem;

    #[test]
    fn it_name() {
        assert_eq!(FromItem::NAMED, "from item");
        assert_eq!(FromItem::NAME(NameExpr::new("")).name(), "name expression");
    }

    #[test]
    fn it_parse() {
        // lexers is end
        let mut lexers: Vec<Lexer> = vec![];
        let mut offset = Offset::new(0);
        assert!(
            matches!(FromItem::parse(&lexers, &mut offset), Err(err) if err.msg == "SyntaxError: missing from item")
        );
        assert_eq!(offset.value, 0);
        // no from item in lexers
        lexers = vec![Lexer::SELECT(String::from("SELECT"))];
        offset.value = 0;
        assert!(
            matches!(FromItem::parse(&lexers, &mut offset), Err(err) if err.msg == "SyntaxError: missing from item")
        );
        assert_eq!(offset.value, 0);
        // contain from item -> name expression
        lexers = vec![Lexer::NAME(String::from("3210"))];
        offset.value = 0;
        assert!(
            matches!(FromItem::parse(&lexers, &mut offset), Ok(item) if matches!(&item, FromItem::NAME(expr) if expr.value == "3210"))
        );
        assert_eq!(offset.value, 1);
    }
}
