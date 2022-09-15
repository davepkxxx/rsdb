use std::array::IntoIter;

use regex::Regex;
use rsdb::NamedEnum;

use super::err::SyntaxError;

/// The lexers of SQL
#[derive(Debug, Clone)]
pub enum Lexer {
    SELECT(String),
    FROM(String),
    AS(String),
    STAR(String),
    COMMA(String),
    WHITESPACE(String),
    NAME(String),
}

impl From<Lexer> for Regex {
    /// Converts to this type from the input type.
    fn from(lexer: Lexer) -> Self {
        let re = match lexer {
            Lexer::SELECT(_) => "^(?i)SELECT",
            Lexer::FROM(_) => "^(?i)FROM",
            Lexer::AS(_) => r"^(?i)AS",
            Lexer::STAR(_) => r"^\*",
            Lexer::COMMA(_) => "^,",
            Lexer::WHITESPACE(_) => r"^[\s]+",
            Lexer::NAME(_) => r"^[A-Za-z][\w]*",
        };
        Self::new(re).unwrap()
    }
}

impl PartialEq for Lexer {
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::SELECT(_) => matches!(other, Self::SELECT(_)),
            Self::FROM(_) => matches!(other, Self::FROM(_)),
            Self::AS(_) => matches!(other, Self::AS(_)),
            Self::STAR(_) => matches!(other, Self::STAR(_)),
            Self::COMMA(_) => matches!(other, Self::COMMA(_)),
            Self::WHITESPACE(_) => matches!(other, Self::WHITESPACE(_)),
            Self::NAME(one) => matches!(other, Self::NAME(other) if one == other),
        }
    }
}

impl Eq for Lexer {}

impl NamedEnum for Lexer {
    fn name(&self) -> &'_ str {
        match self {
            Self::SELECT(_) => "SELECT",
            Self::FROM(_) => "FROM",
            Self::AS(_) => "AS",
            Self::STAR(_) => "STAR",
            Self::COMMA(_) => "COMMA",
            Self::WHITESPACE(_) => "WHITESPACE",
            Self::NAME(_) => "NAME",
        }
    }
}

impl Lexer {
    const VALUES: [Self; 7] = [
        Self::SELECT(String::new()),
        Self::FROM(String::new()),
        Self::AS(String::new()),
        Self::STAR(String::new()),
        Self::COMMA(String::new()),
        Self::WHITESPACE(String::new()),
        Self::NAME(String::new()),
    ];

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the array (from start to end). The array cannot be used after calling
    /// this unless `T` implements `Copy`, so the whole array is copied.
    ///
    /// Arrays have special behavior when calling `.into_iter()` prior to the
    /// 2021 edition -- see the [array] Editions section for more information.
    ///
    /// [array]: prim@array
    pub fn into_iter() -> IntoIter<Lexer, 7> {
        Self::VALUES.into_iter()
    }

    pub fn is(&self, other: &Self) -> bool {
        match self {
            Self::SELECT(_) => matches!(other, Self::SELECT(_)),
            Self::FROM(_) => matches!(other, Self::FROM(_)),
            Self::AS(_) => matches!(other, Self::AS(_)),
            Self::STAR(_) => matches!(other, Self::STAR(_)),
            Self::COMMA(_) => matches!(other, Self::COMMA(_)),
            Self::WHITESPACE(_) => matches!(other, Self::WHITESPACE(_)),
            Self::NAME(_) => matches!(other, Self::NAME(_)),
        }
    }

    fn is_value_ignores(&self) -> bool {
        matches!(
            self,
            Self::SELECT(_) | Self::FROM(_) | Self::STAR(_) | Self::COMMA(_) | Self::WHITESPACE(_)
        )
    }

    pub fn is_clause(&self) -> bool {
        matches!(self, Self::SELECT(_) | Self::FROM(_))
    }

    pub fn value(&self) -> &str {
        match self {
            Self::SELECT(value) => value,
            Self::FROM(value) => value,
            Self::AS(value) => value,
            Self::STAR(value) => value,
            Self::COMMA(value) => value,
            Self::WHITESPACE(value) => value,
            Self::NAME(value) => value,
        }
    }

    /// Returns the length of `self` value.
    ///
    /// This length is in bytes, not [`char`]s or graphemes. In other words,
    /// it might not be what a human considers the length of the string.
    ///
    /// [`char`]: prim@char
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let len = Lexer::NAME("foo").len();
    /// assert_eq!(3, len);
    ///
    /// assert_eq!(Lexer::NAME("foo").len(), 4); // fancy f!
    /// assert_eq!(Lexer::NAME("foo").value().chars().count(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.value().len()
    }

    fn replace(&self, new_value: String) -> Self {
        match self {
            Self::SELECT(_) => Self::SELECT(new_value),
            Self::FROM(_) => Self::FROM(new_value),
            Self::AS(_) => Self::AS(new_value),
            Self::STAR(_) => Self::STAR(new_value),
            Self::COMMA(_) => Self::COMMA(new_value),
            Self::WHITESPACE(_) => Self::WHITESPACE(new_value),
            Self::NAME(_) => Self::NAME(new_value),
        }
    }

    /// Returns the start and end byte range of the leftmost-first match in
    /// `text`. If no match exists, then `None` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() {
    /// let text = "SELECT * FROM table";
    /// let mat = Lexer::SELECT(String::new()).find(text);
    /// assert_matches!(mat, Lexer::SELECT(_));
    /// # }
    /// ```
    fn find(&self, expr: &str) -> Option<Self> {
        match Regex::from(self.clone()).find(expr) {
            Some(m) => Some(self.replace(String::from(m.as_str()))),
            None => None,
        }
    }

    /// Returns the start and end byte range of the leftmost-first match in
    /// `text`. If no match exists, then `None` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() {
    /// let text = "SELECT * FROM table";
    /// let mat = Lexer::find_one(text);
    /// assert_matches!(mat, Lexer::SELECT(_));
    /// # }
    /// ```
    fn find_one(expr: &str) -> Option<Self> {
        Self::into_iter()
            .map_while(|lexer| Some(lexer.find(expr)))
            .fold(None as Option<Self>, |prev, curr| match curr {
                Some(curr_lexer) => match prev {
                    Some(prev_lexer) => {
                        if curr_lexer.len() > prev_lexer.len() {
                            Some(curr_lexer)
                        } else if curr_lexer.len() < prev_lexer.len() {
                            Some(prev_lexer)
                        } else if curr_lexer.is_value_ignores() {
                            Some(curr_lexer)
                        } else {
                            Some(prev_lexer)
                        }
                    }
                    None => Some(curr_lexer),
                },
                None => prev,
            })
    }

    pub fn parse(sql: &str) -> Result<Vec<Self>, SyntaxError> {
        let mut lexers: Vec<Self> = vec![];
        let mut i: usize = 0;

        while i < sql.len() {
            let part = &sql[i..];
            match Self::find_one(part) {
                Some(lexer) => {
                    i += lexer.len();
                    if !matches!(lexer, Self::WHITESPACE(_)) {
                        lexers.push(lexer);
                    }
                }
                None => {
                    return Err(SyntaxError {
                        msg: format!("SyntaxError: expected {}", &sql[i..i + 1]),
                    })
                }
            }
        }

        Ok(lexers)
    }
}

#[cfg(test)]
mod tests {
    use rsdb::NamedEnum;

    use crate::sql::lexer::Lexer;

    #[test]
    fn it_eq() {
        // same value
        assert!(Lexer::SELECT(String::from("0844")).eq(&Lexer::SELECT(String::from("0844"))));
        assert!(Lexer::FROM(String::from("6d74")).eq(&Lexer::FROM(String::from("6d74"))));
        assert!(Lexer::AS(String::from("a839")).eq(&Lexer::AS(String::from("a839"))));
        assert!(Lexer::STAR(String::from("b55c")).eq(&Lexer::STAR(String::from("b55c"))));
        assert!(Lexer::COMMA(String::from("fd3e")).eq(&Lexer::COMMA(String::from("fd3e"))));
        assert!(
            Lexer::WHITESPACE(String::from("1af1")).eq(&Lexer::WHITESPACE(String::from("1af1")))
        );
        assert!(Lexer::NAME(String::from("4ec4")).eq(&Lexer::NAME(String::from("4ec4"))));
        // diff value & ignore values
        assert!(Lexer::SELECT(String::from("8ff6")).eq(&Lexer::SELECT(String::from("4acc"))));
        assert!(Lexer::FROM(String::from("ac16")).eq(&Lexer::FROM(String::from("0a6a"))));
        assert!(Lexer::AS(String::from("2703")).eq(&Lexer::AS(String::from("2703"))));
        assert!(Lexer::STAR(String::from("0bad")).eq(&Lexer::STAR(String::from("327f"))));
        assert!(Lexer::COMMA(String::from("a4f0")).eq(&Lexer::COMMA(String::from("4cfe"))));
        assert!(
            Lexer::WHITESPACE(String::from("80e2")).eq(&Lexer::WHITESPACE(String::from("c2c5")))
        );
        // diff value & match values
        assert!(!Lexer::NAME(String::from("7133")).eq(&Lexer::NAME(String::from("5d5e"))));
    }

    #[test]
    fn it_name() {
        assert_eq!(Lexer::SELECT(String::new()).name(), "SELECT");
        assert_eq!(Lexer::FROM(String::new()).name(), "FROM");
        assert_eq!(Lexer::AS(String::new()).name(), "AS");
        assert_eq!(Lexer::STAR(String::new()).name(), "STAR");
        assert_eq!(Lexer::COMMA(String::new()).name(), "COMMA");
        assert_eq!(Lexer::WHITESPACE(String::new()).name(), "WHITESPACE");
        assert_eq!(Lexer::NAME(String::new()).name(), "NAME");
    }

    #[test]
    fn it_value() {
        assert_eq!(Lexer::SELECT(String::from("cc45")).value(), "cc45");
        assert_eq!(Lexer::FROM(String::from("d733")).value(), "d733");
        assert_eq!(Lexer::AS(String::from("46c6")).value(), "46c6");
        assert_eq!(Lexer::STAR(String::from("debb")).value(), "debb");
        assert_eq!(Lexer::COMMA(String::from("41ce")).value(), "41ce");
        assert_eq!(Lexer::WHITESPACE(String::from("b734")).value(), "b734");
        assert_eq!(Lexer::NAME(String::from("ee4f")).value(), "ee4f");
    }

    #[test]
    fn it_len() {
        assert_eq!(Lexer::SELECT(String::from("ea8")).len(), 3);
        assert_eq!(Lexer::FROM(String::from("82861")).len(), 5);
        assert_eq!(Lexer::AS(String::from("81ea4155")).len(), 8);
        assert_eq!(Lexer::STAR(String::from("0286")).len(), 4);
        assert_eq!(Lexer::COMMA(String::from("272")).len(), 3);
        assert_eq!(Lexer::WHITESPACE(String::from("eab84")).len(), 5);
        assert_eq!(Lexer::NAME(String::from("44")).len(), 2);
    }

    #[test]
    fn it_into_iter() {
        assert_eq!(Lexer::into_iter().len(), 7);
        assert!(Lexer::into_iter().any(|lexer| matches!(lexer, Lexer::SELECT(_))));
        assert!(Lexer::into_iter().any(|lexer| matches!(lexer, Lexer::FROM(_))));
        assert!(Lexer::into_iter().any(|lexer| matches!(lexer, Lexer::AS(_))));
        assert!(Lexer::into_iter().any(|lexer| matches!(lexer, Lexer::STAR(_))));
        assert!(Lexer::into_iter().any(|lexer| matches!(lexer, Lexer::COMMA(_))));
        assert!(Lexer::into_iter().any(|lexer| matches!(lexer, Lexer::WHITESPACE(_))));
        assert!(Lexer::into_iter().any(|lexer| matches!(lexer, Lexer::NAME(_))));
    }

    #[test]
    fn it_find() {
        // Lexer::SELECT
        assert!(matches!(
            Lexer::SELECT(String::new()).find("SELECT"),
            Some(lexer) if matches!(&lexer, Lexer::SELECT(value) if value == "SELECT")
        ));
        assert!(matches!(
            Lexer::SELECT(String::new()).find("select"),
            Some(lexer) if matches!(&lexer, Lexer::SELECT(value) if value == "select")
        ));
        // Lexer::FROM
        assert!(matches!(
            Lexer::FROM(String::new()).find("FROM"),
            Some(lexer) if matches!(&lexer, Lexer::FROM(value) if value == "FROM")
        ));
        assert!(matches!(
            Lexer::FROM(String::new()).find("from"),
            Some(lexer) if matches!(&lexer, Lexer::FROM(value) if value == "from")
        ));
        // Lexer::AS
        assert!(matches!(
            Lexer::AS(String::new()).find("AS"),
            Some(lexer) if matches!(&lexer, Lexer::AS(value) if value == "AS")
        ));
        assert!(matches!(
            Lexer::AS(String::new()).find("as"),
            Some(lexer) if matches!(&lexer, Lexer::AS(value) if value == "as")
        ));
        // Lexer::STAR
        assert!(matches!(
            Lexer::STAR(String::new()).find("*"),
            Some(lexer) if matches!(&lexer, Lexer::STAR(value) if value == "*")
        ));
        // Lexer::COMMA
        assert!(matches!(
            Lexer::COMMA(String::new()).find(","),
            Some(lexer) if matches!(&lexer, Lexer::COMMA(value) if value == ",")
        ));
        // Lexer::WHITESPACE
        assert!(matches!(
            Lexer::WHITESPACE(String::new()).find(" \r\n\t"),
            Some(lexer) if matches!(&lexer, Lexer::WHITESPACE(value) if value == " \r\n\t")
        ));
        // Lexer::NAME
        assert!(matches!(
            Lexer::NAME(String::new()).find("aA1_"),
            Some(lexer) if matches!(&lexer, Lexer::NAME(value) if value == "aA1_")
        ));
        assert!(matches!(
            Lexer::NAME(String::new()).find("Aa1_"),
            Some(lexer) if matches!(&lexer, Lexer::NAME(value) if value == "Aa1_")
        ));
        assert!(matches!(Lexer::NAME(String::new()).find("_aA1"), None));
        assert!(matches!(Lexer::NAME(String::new()).find("1aA_"), None));
    }

    #[test]
    fn it_find_one() {
        // Lexer::SELECT
        assert!(matches!(
            Lexer::find_one("SELECT * FROM TABLE_1"),
            Some(lexer) if matches!(&lexer, Lexer::SELECT(value) if value == "SELECT")
        ));
        assert!(matches!(
            Lexer::find_one("select * from table_1"),
            Some(lexer) if matches!(&lexer, Lexer::SELECT(value) if value == "select")
        ));
        // Lexer::FROM
        assert!(matches!(
            Lexer::find_one("FROM TABLE_1 WHERE 1=1"),
            Some(lexer) if matches!(&lexer, Lexer::FROM(value) if value == "FROM")
        ));
        assert!(matches!(
            Lexer::find_one("from table_1 where 1=1"),
            Some(lexer) if matches!(&lexer, Lexer::FROM(value) if value == "from")
        ));
        // Lexer::AS
        assert!(matches!(
            Lexer::find_one("AS ALL_COLUMNS FROM TABLE_1"),
            Some(lexer) if matches!(&lexer, Lexer::AS(value) if value == "AS")
        ));
        assert!(matches!(
            Lexer::find_one("as all_columns from table_1"),
            Some(lexer) if matches!(&lexer, Lexer::AS(value) if value == "as")
        ));
        // Lexer::STAR
        assert!(matches!(
            Lexer::find_one("* from table_1"),
            Some(lexer) if matches!(&lexer, Lexer::STAR(value) if value == "*")
        ));
        // Lexer::COMMA
        assert!(matches!(
            Lexer::find_one(",* from table_1"),
            Some(lexer) if matches!(&lexer, Lexer::COMMA(value) if value == ",")
        ));
        // Lexer::WHITESPACE
        assert!(matches!(
            Lexer::find_one(" \r\n\t* from table_1"),
            Some(lexer) if matches!(&lexer, Lexer::WHITESPACE(value) if value == " \r\n\t")
        ));
        // Lexer::NAME
        assert!(matches!(
            Lexer::find_one("aA1_ from table_1"),
            Some(lexer) if matches!(&lexer, Lexer::NAME(value) if value == "aA1_")
        ));
        assert!(matches!(
            Lexer::find_one("Aa1_ from table_1"),
            Some(lexer) if matches!(&lexer, Lexer::NAME(value) if value == "Aa1_")
        ));
        assert!(matches!(Lexer::find_one("_aA1 from table_1"), None));
        assert!(matches!(Lexer::find_one("1aA_ from table_1"), None));
    }

    #[test]
    fn it_parse() {
        // lower case
        assert!(matches!(
            Lexer::parse("select * from table_1"),
            Ok(lexers) if
                lexers.len() == 4 &&
                matches!(lexers.get(0), Some(lexer) if matches!(lexer, Lexer::SELECT(value) if value == "select")) &&
                matches!(lexers.get(1), Some(lexer) if matches!(lexer, Lexer::STAR(value) if value == "*")) &&
                matches!(lexers.get(2), Some(lexer) if matches!(lexer, Lexer::FROM(value) if value == "from")) &&
                matches!(lexers.get(3), Some(lexer) if matches!(lexer, Lexer::NAME(value) if value == "table_1"))
        ));
        // upper case
        assert!(matches!(
            Lexer::parse("SELECT * FROM TABLE_1"),
            Ok(lexers) if
                lexers.len() == 4 &&
                matches!(lexers.get(0), Some(lexer) if matches!(lexer, Lexer::SELECT(value) if value == "SELECT")) &&
                matches!(lexers.get(1), Some(lexer) if matches!(lexer, Lexer::STAR(value) if value == "*")) &&
                matches!(lexers.get(2), Some(lexer) if matches!(lexer, Lexer::FROM(value) if value == "FROM")) &&
                matches!(lexers.get(3), Some(lexer) if matches!(lexer, Lexer::NAME(value) if value == "TABLE_1"))
        ));
        // err
        assert!(matches!(
            Lexer::parse("select * from _table1"),
            Err(err) if err.msg.eq("SyntaxError: expected _")
        ));
    }
}
