use rsdb::{Named, Offset};

use crate::sql::{parser::LexerParser, lexer::Lexer, err::SyntaxError};

#[derive(Debug, Clone)]
pub struct NameExpr {
    pub value: String,
}

impl Named for NameExpr {
    const NAMED: &'static str = "name expression";
}

impl LexerParser for NameExpr {
    fn parse(source: &Vec<Lexer>, offset: &mut Offset) -> Result<Self, SyntaxError>
    where
        Self: Sized,
    {
        match source.get(offset.value) {
            Some(lexer) => match lexer {
                Lexer::NAME(value) => {
                    offset.increment(1);
                    Ok(NameExpr::new(value))
                }
                _ => Err(SyntaxError::new_missing(Self::NAMED)),
            },
            None => Err(SyntaxError::new_missing(Self::NAMED)),
        }
    }
}

impl NameExpr {
    pub fn new(value: &str) -> Self {
        NameExpr {
            value: String::from(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use rsdb::{Named, Offset};

    use crate::sql::{lexer::Lexer, parser::LexerParser};

    use super::NameExpr;

    #[test]
    fn it_name() {
        assert_eq!(NameExpr::NAMED, "name expression");
    }

    #[test]
    fn it_new() {
        assert_eq!(NameExpr::new("4812").value, "4812");
    }

    #[test]
    fn it_parse() {
        // lexers is end
        let mut source: Vec<Lexer> = vec![];
        let mut offset = Offset::new(0);
        assert!(matches!(NameExpr::parse(&source, &mut offset), Err(err) if err.msg == "SyntaxError: missing name expression"));
        // current lexer is not a name expression
        source = vec![Lexer::SELECT(String::from("SELECT"))];
        offset.value = 0;
        assert!(matches!(NameExpr::parse(&source, &mut offset), Err(err) if err.msg == "SyntaxError: missing name expression"));
        // current lexer is not a name expression
        source = vec![Lexer::NAME(String::from("8d06"))];
        offset.value = 0;
        assert!(matches!(NameExpr::parse(&source, &mut offset), Ok(expr) if expr.value == "8d06"));
    }
}
