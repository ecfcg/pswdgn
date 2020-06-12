pub(crate) mod character;
pub mod cli;
pub mod error;

use self::character::CharSets;
use self::error::Error;

use rand::seq::SliceRandom;

pub(crate) const MIN_LENGTH: i128 = 8;
pub(crate) const MAX_LENGTH: i128 = std::u8::MAX as i128;

#[macro_export]
macro_rules! symbols_all {
    () => {
        r##"!@#$%^&*()\=+_-{}[]:`~|'"<>?;/.,"##
    };
}

pub struct Generator {
    length: usize,
    usable: CharSets,
}

impl Generator {
    pub fn from_cli(
        length: usize,
        flag_str: String,
        is_easy: bool,
        symbols: String,
    ) -> Result<Self, Error> {
        Self::new(length, CharSets::from_cli(flag_str, is_easy, symbols)?)
    }

    pub fn from_code(
        length: usize,
        code: usize,
        is_easy: bool,
        symbols: String,
    ) -> Result<Self, Error> {
        Self::new(length, CharSets::from_code(code, is_easy, symbols)?)
    }

    fn new(length: usize, usable: CharSets) -> Result<Self, Error> {
        Self::validate_length(length as i128)?;
        Ok(Generator {
            length: length,
            usable: usable,
        })
    }

    pub fn generate(self: &Self) -> String {
        let mut generated: String;
        loop {
            generated = self.generate_str();
            if self.usable.exists_intersection(&generated) {
                break;
            }
        }
        generated
    }

    fn generate_str(self: &Self) -> String {
        let characters = self
            .usable
            .characters()
            .iter()
            .map(|c| *c)
            .collect::<Vec<char>>();
        let mut rng = rand::thread_rng();
        let mut s = String::with_capacity(self.length);

        for _ in 0..self.length {
            let c = characters.choose(&mut rng).unwrap();
            s.push(*c);
        }
        s
    }

    pub(crate) fn validate_length(length: i128) -> Result<(), error::Error> {
        if length < MIN_LENGTH {
            Err(error::Error::LengthInsufficientErr(length))
        } else if MAX_LENGTH < length {
            Err(error::Error::LengthExcessErr(length))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_cli() {
        let gen = Generator::from_cli(8, String::from("luns"), true, String::from("!@#$%"))
            .ok()
            .unwrap();
        assert_eq!(gen.length, 8);
        assert_eq!(
            gen.usable,
            CharSets::from_cli(String::from("luns"), true, String::from("!@#$%"))
                .ok()
                .unwrap()
        );
    }

    #[test]
    fn test_new() {
        let gen = Generator::new(
            8,
            CharSets::from_cli(String::from("l"), false, String::default())
                .ok()
                .unwrap(),
        )
        .ok()
        .unwrap();
        assert_eq!(gen.length, 8);
        assert_eq!(
            gen.usable,
            CharSets::from_cli(String::from("l"), false, String::default())
                .ok()
                .unwrap(),
        );
    }

    #[test]
    fn test_generate() {
        let gen = Generator::new(
            8,
            CharSets::from_cli(String::from("l"), false, String::default())
                .ok()
                .unwrap(),
        )
        .ok()
        .unwrap();
        let result = gen.generate();
        assert_eq!(result.len(), 8);
        assert_eq!(gen.usable.exists_intersection(&result), true);
    }

    #[test]
    fn test_generate_str() {
        let gen = Generator::new(
            10,
            CharSets::from_cli(String::from("l"), false, String::default())
                .ok()
                .unwrap(),
        )
        .ok()
        .unwrap();
        let result = gen.generate();
        assert_eq!(result.len(), 10);
        assert_ne!(result, gen.generate());
        assert_ne!(result, gen.generate());
    }

    #[test]
    fn test_validate_length() {
        assert_eq!(
            Generator::validate_length(7),
            Err(error::Error::LengthInsufficientErr(7))
        );
        assert_eq!(Generator::validate_length(8), Ok(()));
        assert_eq!(Generator::validate_length(255), Ok(()));
        assert_eq!(
            Generator::validate_length(256),
            Err(error::Error::LengthExcessErr(256))
        );
    }
}
