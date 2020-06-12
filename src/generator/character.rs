mod category;

pub(crate) use self::category::Category;
use crate::generator::error::Error;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub(crate) struct CharSets {
    char_sets: Vec<HashSet<char>>,
}

impl CharSets {
    pub(crate) fn from_cli(
        flag_str: String,
        is_easy: bool,
        symbols: String,
    ) -> Result<Self, Error> {
        Ok(Self::new(Category::from_cli(flag_str)?, is_easy, symbols))
    }

    pub(crate) fn from_code(code: usize, is_easy: bool, symbols: String) -> Result<Self, Error> {
        Ok(Self::new(Category::from_code(code)?, is_easy, symbols))
    }

    fn new(char_sets: Vec<&'static Category>, is_easy: bool, symbols: String) -> Self {
        CharSets {
            char_sets: char_sets
                .iter()
                .map(|cs| cs.char_set(is_easy, &symbols))
                .collect(),
        }
    }

    pub(crate) fn characters(self: &Self) -> HashSet<char> {
        self.char_sets
            .iter()
            .flat_map(|c| c.iter())
            .map(|c| *c)
            .collect()
    }

    pub(crate) fn exists_intersection(self: &Self, str: &String) -> bool {
        let c_set = str.chars().into_iter().collect();
        self.char_sets.iter().all(|c| !c.is_disjoint(&c_set))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_cli() {
        let cs = CharSets::from_cli(String::from("luns"), true, String::from("!@#$%"))
            .ok()
            .unwrap();
        assert_eq!(
            cs.char_sets,
            vec![
                category::LOWER.char_set(true, &String::default()),
                category::UPPER.char_set(true, &String::default()),
                category::NUMBER.char_set(true, &String::default()),
                category::SYMBOL.char_set(true, &String::from("!@#$%")),
            ]
        );
    }

    #[test]
    fn test_from_code() {
        let cs = CharSets::from_code(15, true, String::from("!@#$%"))
            .ok()
            .unwrap();
        assert_eq!(
            cs.char_sets,
            vec![
                category::LOWER.char_set(true, &String::default()),
                category::UPPER.char_set(true, &String::default()),
                category::NUMBER.char_set(true, &String::default()),
                category::SYMBOL.char_set(true, &String::from("!@#$%")),
            ]
        );
    }

    #[test]
    fn test_new() {
        let cs = CharSets::new(vec![&category::LOWER], false, String::default());
        assert_eq!(
            cs.char_sets,
            vec![category::LOWER.char_set(false, &String::default())]
        );
    }

    #[test]
    fn test_characters() {
        let cs = CharSets::new(
            vec![&category::LOWER, &category::UPPER],
            false,
            String::default(),
        );
        assert_eq!(
            cs.characters(),
            (&category::LOWER.char_set(false, &String::default())
                | &category::UPPER.char_set(false, &String::default()))
        );
    }

    #[test]
    fn test_exists_intersection() {
        let cs = CharSets::new(
            vec![&category::LOWER, &category::UPPER],
            false,
            String::default(),
        );
        assert_eq!(cs.exists_intersection(&String::from("aA1!")), true);
        assert_eq!(cs.exists_intersection(&String::from("a1!")), false);
        assert_eq!(cs.exists_intersection(&String::from("A1!")), false);
        assert_eq!(cs.exists_intersection(&String::from("1!")), false);
    }
}
