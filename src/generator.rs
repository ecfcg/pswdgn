pub mod character;
pub mod cli;
pub mod constants;

use character::UsableCharacters;
use rand::random;

pub(crate) enum LengthErr {
    OVER,
    UNDER,
}

pub(crate) fn validate_length(length: i128) -> Result<(), LengthErr> {
    if constants::MAX_LENGTH < length {
        return Err(LengthErr::OVER);
    }
    if length < constants::MIN_LENGTH {
        return Err(LengthErr::UNDER);
    }
    Ok(())
}

pub struct Generator {
    length: u8,
    param: UsableCharacters,
}

impl Generator {
    pub fn new(length: u8, usable_code: usize, is_easy: bool) -> Self {
        let checked_length = if length < 8 { 8 } else { length };

        Generator {
            length: checked_length,
            param: UsableCharacters::new(usable_code, is_easy),
        }
    }

    pub fn from_cli(command_line: cli::CommandLine) -> Self {
        Self::new(
            command_line.length,
            command_line.usable_code,
            command_line.is_easy,
        )
    }

    pub fn generate(self: &Self) -> String {
        let mut generated: String;

        loop {
            generated = self.generate_str();
            if self.param.contains_all_usable_characters(&generated) {
                break;
            }
        }
        generated
    }

    fn generate_str(self: &Self) -> String {
        let chars = self
            .param
            .get_usable_characters()
            .chars()
            .collect::<Vec<char>>();
        let mut generated = String::with_capacity(self.length as usize);
        for _i in 0..self.length {
            let index = random::<usize>() % chars.len();
            generated.push(chars[index]);
        }
        generated
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Generator::new(7, 15, true).length, 8);
        assert_eq!(Generator::new(8, 15, true).length, 8);
        assert_eq!(Generator::new(9, 15, true).length, 9);
    }

    #[test]
    fn test_generate() {
        let generator = Generator::new(8, 7, true);
        let result = generator.generate();
        assert_eq!(result.len(), 8);
        assert_eq!(
            generator.param.contains_all_usable_characters(&result),
            true
        )
    }

    #[test]
    fn test_generate_str() {
        let generator = Generator::new(10, 4, true);
        let result = generator.generate();
        assert_eq!(result.len(), 10);
        assert_ne!(result, generator.generate());
        assert_ne!(result, generator.generate());
    }
}
