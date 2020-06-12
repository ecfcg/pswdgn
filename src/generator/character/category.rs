use crate::generator::error::Error;
use std::collections::HashSet;

/// Category of characters for generate password.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Category {
    pub(crate) chars_all: &'static str,
    chars_easy: &'static str,
    flag: char,
    code_point: usize,
}

/// Lower case alphabets.
pub(crate) const LOWER: Category = Category {
    chars_all: "abcdefghijklmnopqrstuvwxyz",
    chars_easy: "abcdefghijkmnpqrstuvwxyz",
    flag: 'l',
    code_point: 0,
};

/// Upper case alphabets.
pub(crate) const UPPER: Category = Category {
    chars_all: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    chars_easy: "ABCDEFGHJKLMNPQRSTUVWXYZ",
    flag: 'u',
    code_point: 1,
};

/// Numbers.
pub(crate) const NUMBER: Category = Category {
    chars_all: "0123456789",
    chars_easy: "23456789",
    flag: 'n',
    code_point: 2,
};

/// Symbols.
pub(crate) const SYMBOL: Category = Category {
    chars_all: crate::symbols_all!(),
    chars_easy: r##"!@#$%^&*=+~"<>?"##,
    flag: 's',
    code_point: 3,
};

const ALL_CHARACTERS: [Category; 4] = [LOWER, UPPER, NUMBER, SYMBOL];

impl Category {
    pub(crate) fn from_cli(flag_str: String) -> Result<Vec<&'static Self>, Error> {
        let v = Self::new(&|c| flag_str.contains(c.flag))?;
        Ok(v)
    }

    pub(crate) fn from_code(code: usize) -> Result<Vec<&'static Self>, Error> {
        let v = Self::new(&|c| code & c.code() == c.code())?;
        Ok(v)
    }

    fn new(p: &dyn Fn(&&Self) -> bool) -> Result<Vec<&'static Self>, Error> {
        let categorys: Vec<&'static Self> = ALL_CHARACTERS.iter().filter(p).collect();
        if categorys.is_empty() {
            Err(Error::CharactersErr(()))
        } else {
            Ok(categorys)
        }
    }

    pub(crate) fn flags() -> String {
        ALL_CHARACTERS.iter().map(|c| c.flag).collect()
    }

    fn code(self: &Self) -> usize {
        1 << self.code_point
    }

    pub(crate) fn char_set(self: &Self, is_easy: bool, symbols: &String) -> HashSet<char> {
        let characters = if SYMBOL == *self && !symbols.is_empty() {
            symbols
        } else if is_easy {
            self.chars_easy
        } else {
            self.chars_all
        };

        characters.chars().into_iter().collect()
    }

    pub(crate) fn validate_flag(frag_str: &String) -> Result<(), Error> {
        let flags = Self::flags();
        let errors: String = frag_str
            .chars()
            .into_iter()
            .filter(|f| !flags.contains(*f))
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::CategoryFlagErr(errors))
        }
    }

    pub(crate) fn validate_symbols(symbols: &String) -> Result<(), Error> {
        let symbol_all = SYMBOL.chars_all;
        let errors: String = symbols
            .chars()
            .into_iter()
            .filter(|s| !symbol_all.contains(*s))
            .collect();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::NotSymbolErr(errors))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_from_cli() {
        assert_eq!(
            Category::from_cli(String::from("luns")).ok().unwrap(),
            vec![&LOWER, &UPPER, &NUMBER, &SYMBOL]
        );

        assert_eq!(
            Category::from_cli(String::from("")).err().unwrap(),
            Error::CharactersErr(())
        );
    }

    #[test]
    fn test_from_code() {
        assert_eq!(
            Category::from_code(15).ok().unwrap(),
            vec![&LOWER, &UPPER, &NUMBER, &SYMBOL]
        );

        assert_eq!(
            Category::from_code(0).err().unwrap(),
            Error::CharactersErr(())
        );
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Category::new(&|_| true).ok().unwrap(),
            vec![&LOWER, &UPPER, &NUMBER, &SYMBOL]
        );

        assert_eq!(
            Category::new(&|_| false).err().unwrap(),
            Error::CharactersErr(())
        );
    }

    #[test]
    fn test_flags() {
        assert_eq!(Category::flags(), "luns");
    }

    #[test]
    fn test_code() {
        assert_eq!(LOWER.code(), 1);
        assert_eq!(UPPER.code(), 2);
        assert_eq!(NUMBER.code(), 4);
        assert_eq!(SYMBOL.code(), 8);
    }

    #[test]
    fn test_char_set() {
        assert_eq!(
            LOWER.char_set(true, &String::default()),
            HashSet::from_iter(LOWER.chars_easy.chars())
        );
        assert_eq!(
            LOWER.char_set(false, &String::from("!@#$%")),
            HashSet::from_iter(LOWER.chars_all.chars())
        );
        assert_eq!(
            SYMBOL.char_set(true, &String::default()),
            HashSet::from_iter(SYMBOL.chars_easy.chars())
        );
        assert_eq!(
            SYMBOL.char_set(false, &String::from("!@#$%")),
            HashSet::from_iter("!@#$%".chars())
        );
    }

    #[test]
    fn test_validate_flag() {
        assert_eq!(Category::validate_flag(&Category::flags()), Ok(()));
        assert_eq!(
            Category::validate_flag(&String::from("abcs")),
            Err(Error::CategoryFlagErr(String::from("abc")))
        );
    }

    #[test]
    fn test_validate_symbols() {
        assert_eq!(
            Category::validate_symbols(&String::from(SYMBOL.chars_all)),
            Ok(())
        );
        assert_eq!(
            Category::validate_symbols(&String::from("a!@b#$%cs")),
            Err(Error::NotSymbolErr(String::from("abcs")))
        );
    }
}
