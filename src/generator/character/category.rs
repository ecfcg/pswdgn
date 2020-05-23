use super::constants;

/// Category of characters for generate password.
#[derive(Copy, Clone, Debug)]
pub(crate) struct Category {
    chars_all: &'static str,
    chars_easy: &'static str,
    pub flag: &'static str,
    code_point: usize,
}

/// Lower case alphabets.
pub(crate) const LOWER: Category = Category {
    chars_all: constants::LOWER_CHARS_ALL,
    chars_easy: constants::LOWER_CHARS_EASY,
    flag: constants::LOWER_FLAG,
    code_point: 0,
};

/// Upper case alphabets.
pub(crate) const UPPER: Category = Category {
    chars_all: constants::UPPER_CHARS_ALL,
    chars_easy: constants::UPPER_CHARS_EASY,
    flag: constants::UPPER_FLAG,
    code_point: 1,
};

/// Numbers.
pub(crate) const NUMBER: Category = Category {
    chars_all: constants::NUMBER_CHARS_ALL,
    chars_easy: constants::NUMBER_CHARS_EASY,
    flag: constants::NUMBER_FLAG,
    code_point: 2,
};

/// Symbols.
pub(crate) const SYMBOL: Category = Category {
    chars_all: constants::SYMBOL_CHARS_ALL,
    chars_easy: constants::SYMBOL_CHARS_EASY,
    flag: constants::SYMBOL_FLAG,
    code_point: 3,
};

pub(crate) const ALL_CHARACTERS: [Category; 4] = [LOWER, UPPER, NUMBER, SYMBOL];

impl Category {
    pub(crate) fn all_flags() -> String {
        ALL_CHARACTERS.iter().map(|ct| ct.flag).collect::<String>()
    }

    pub(crate) fn all_flagged_code() -> usize {
        ALL_CHARACTERS
            .iter()
            .map(|ct| ct.code())
            .fold(0, |acc, x| acc + x)
    }

    pub(crate) fn flags_to_code(flag_str: &String) -> Result<usize, String> {
        if flag_str.is_empty() {
            return Err(String::from("Argument is empty string."));
        }

        match Self::validate_flag(flag_str) {
            Ok(_) => (),
            Err(err_chars) => return Err(err_chars),
        }

        let code = ALL_CHARACTERS
            .iter()
            .map(|ct| ct.to_code(flag_str))
            .fold(0, |acc, x| acc + x);

        Ok(code)
    }

    pub(crate) fn validate_flag(flag_str: &String) -> Result<(), String> {
        let all_flags = Self::all_flags();
        let mut err_chars = String::with_capacity(flag_str.len());
        for c in flag_str.chars() {
            if !all_flags.contains(c) {
                err_chars.push(c);
            }
        }
        if err_chars.is_empty() {
            Ok(())
        } else {
            Err(err_chars)
        }
    }

    pub(crate) fn validate_code(code: usize) -> Result<(), String> {
        let max = Self::all_flagged_code();
        if code < 1 || max < code {
            Err(format!("Code out of range :{}", code))
        } else {
            Ok(())
        }
    }

    pub(crate) fn get_character(self: &Self, is_easy: bool) -> String {
        if is_easy {
            String::from(self.chars_easy)
        } else {
            String::from(self.chars_all)
        }
    }

    fn to_code(self: &Self, flags: &String) -> usize {
        if flags.contains(self.flag) {
            self.code()
        } else {
            0
        }
    }

    fn code(self: &Self) -> usize {
        1 << self.code_point
    }

    pub(crate) fn is_flagged(self: &Self, code: usize) -> bool {
        code & self.code() == self.code()
    }

    pub(crate) fn exists_intersection(self: &Self, is_easy: bool, s: &String) -> bool {
        for c in self.get_character(is_easy).chars() {
            if s.contains(c) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all_flags() {
        assert_eq!(Category::all_flags(), String::from("luns"));
    }

    #[test]
    fn test_all_flagged_code() {
        assert_eq!(Category::all_flagged_code(), 15 as usize);
    }

    #[test]
    fn test_flags_to_code() {
        assert_eq!(
            Category::flags_to_code(&String::from("luns")),
            Ok(15 as usize)
        );
        assert_eq!(
            Category::flags_to_code(&String::from("anbscd")),
            Err(String::from("abcd"))
        );
        assert_eq!(
            Category::flags_to_code(&String::from("")),
            Err(String::from("Argument is empty string."))
        );
    }

    #[test]
    fn test_assert_code() {
        assert_eq!(Category::validate_code(1), Ok(()));
        assert_eq!(Category::validate_code(15), Ok(()));
        assert_eq!(
            Category::validate_code(0),
            Err(String::from("Code out of range :0"))
        );
        assert_eq!(
            Category::validate_code(16),
            Err(String::from("Code out of range :16"))
        );
    }

    #[test]
    fn test_get_character() {
        assert_eq!(LOWER.get_character(true), LOWER.chars_easy);
        assert_eq!(LOWER.get_character(false), LOWER.chars_all);
    }

    #[test]
    fn test_to_code() {
        assert_eq!(
            LOWER.to_code(&String::from(constants::LOWER_FLAG)),
            LOWER.code()
        );
        assert_eq!(
            UPPER.to_code(&String::from(constants::UPPER_FLAG)),
            UPPER.code()
        );
        assert_eq!(
            NUMBER.to_code(&String::from(constants::NUMBER_FLAG)),
            NUMBER.code()
        );
        assert_eq!(
            SYMBOL.to_code(&String::from(constants::SYMBOL_FLAG)),
            SYMBOL.code()
        );
        assert_eq!(
            LOWER.to_code(&String::from(constants::SYMBOL_FLAG)),
            0 as usize
        );
    }

    #[test]
    fn test_code() {
        assert_eq!(LOWER.code(), 1 as usize);
        assert_eq!(UPPER.code(), 2 as usize);
        assert_eq!(NUMBER.code(), 4 as usize);
        assert_eq!(SYMBOL.code(), 8 as usize);
    }

    #[test]
    fn test_is_flagged() {
        assert_eq!(LOWER.is_flagged(LOWER.code()), true);
        assert_eq!(UPPER.is_flagged(UPPER.code()), true);
        assert_eq!(NUMBER.is_flagged(NUMBER.code()), true);
        assert_eq!(SYMBOL.is_flagged(SYMBOL.code()), true);
        assert_eq!(LOWER.is_flagged(SYMBOL.code()), false);
    }

    #[test]
    fn test_exists_intersection() {
        assert_eq!(LOWER.exists_intersection(true, &String::from("a")), true);
        assert_eq!(LOWER.exists_intersection(true, &String::from("A")), false);
    }
}
