use self::category::{Category, ALL_CHARACTERS};
use super::constants;

pub mod category;

pub struct UsableCharacters {
    usable_code: usize,
    is_easy: bool,
}

impl UsableCharacters {
    pub(crate) fn new(usable_code: usize, is_easy: bool) -> Self {
        match Category::validate_code(usable_code) {
            Ok(_) => (),
            Err(s) => panic!(s),
        }

        UsableCharacters {
            usable_code: usable_code,
            is_easy: is_easy,
        }
    }

    pub(crate) fn get_usable_characters(self: &Self) -> String {
        ALL_CHARACTERS
            .iter()
            .filter(|ct| ct.is_flagged(self.usable_code))
            .map(|ct| ct.get_character(self.is_easy))
            .collect::<String>()
    }

    pub(crate) fn contains_all_usable_characters(self: &Self, s: &String) -> bool {
        ALL_CHARACTERS
            .iter()
            .filter(|ct| ct.is_flagged(self.usable_code))
            .map(|ct| ct.exists_intersection(self.is_easy, s))
            .fold(true, |acc, x| acc && x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let result = UsableCharacters::new(15, true);
        assert_eq!(result.usable_code, 15);
        assert_eq!(result.is_easy, true);
    }

    #[test]
    #[should_panic(expected = "Code out of range :16")]
    fn test_new_panic() {
        UsableCharacters::new(16, true);
    }

    #[test]
    fn test_get_usable_characters() {
        assert_eq!(
            UsableCharacters::new(3, false).get_usable_characters(),
            format!(
                "{}{}",
                constants::LOWER_CHARS_ALL,
                constants::UPPER_CHARS_ALL
            )
        );
    }

    #[test]
    fn test_contains_all_usable_characters() {
        assert_eq!(
            UsableCharacters::new(15, false).contains_all_usable_characters(&String::from("aB3$")),
            true
        );assert_eq!(
            UsableCharacters::new(15, true).contains_all_usable_characters(&String::from("aB0$")),
            false
        );
    }
}
