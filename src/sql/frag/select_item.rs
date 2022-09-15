use rsdb::{Named, NamedEnum, Offset};

use crate::sql::{
    err::SyntaxError,
    expr::{alias::AliasParser, name::NameExpr},
    lexer::Lexer,
    parser::LexerParser,
};

#[derive(Debug, Clone)]
pub enum SelectItem {
    NAME(NameExpr),
    STAR,
}

impl Named for SelectItem {
    const NAMED: &'static str = "select item";
}

impl NamedEnum for SelectItem {
    fn name(&self) -> &'_ str {
        match self {
            Self::NAME(_) => NameExpr::NAMED,
            Self::STAR => "STAR",
        }
    }
}

impl LexerParser for SelectItem {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError> {
        match source.get(offset.value) {
            Some(lexer) => match lexer {
                Lexer::NAME(_) => Ok(SelectItem::NAME(NameExpr::parse(source, offset).unwrap())),
                Lexer::STAR(_) => {
                    offset.increment(1);
                    Ok(SelectItem::STAR)
                }
                _ => Err(SyntaxError::new_missing(SelectItem::NAMED)),
            },
            None => Err(SyntaxError::new_missing(SelectItem::NAMED)),
        }
    }
}

impl AliasParser for SelectItem {}

#[cfg(test)]
mod tests {
    use rsdb::{Named, NamedEnum, Offset};

    use crate::sql::{expr::name::NameExpr, lexer::Lexer, parser::LexerParser};

    use super::SelectItem;

    #[test]
    fn it_name() {
        assert_eq!(SelectItem::NAMED, "select item");
        assert_eq!(
            SelectItem::NAME(NameExpr::new("")).name(),
            "name expression"
        );
        assert_eq!(SelectItem::STAR.name(), "STAR");
    }

    #[test]
    fn it_parse() {
        // lexers is end
        let mut lexers: Vec<Lexer> = vec![];
        let mut offset = Offset::new(0);
        assert!(
            matches!(SelectItem::parse(&lexers, &mut offset), Err(err) if err.msg == "SyntaxError: missing select item")
        );
        assert_eq!(offset.value, 0);
        // no from item in lexers
        lexers = vec![Lexer::SELECT(String::from("SELECT"))];
        offset.value = 0;
        assert!(
            matches!(SelectItem::parse(&lexers, &mut offset), Err(err) if err.msg == "SyntaxError: missing select item")
        );
        assert_eq!(offset.value, 0);
        // contain from item -> name expression
        lexers = vec![Lexer::NAME(String::from("4dfa"))];
        offset.value = 0;
        assert!(
            matches!(SelectItem::parse(&lexers, &mut offset), Ok(item) if matches!(&item, SelectItem::NAME(expr) if expr.value == "4dfa"))
        );
        assert_eq!(offset.value, 1);
        // contain from item -> name expression
        lexers = vec![Lexer::STAR(String::from("*"))];
        offset.value = 0;
        assert!(
            matches!(SelectItem::parse(&lexers, &mut offset), Ok(item) if matches!(&item, SelectItem::STAR))
        );
        assert_eq!(offset.value, 1);
    }
}
