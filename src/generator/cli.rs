use clap::{App, Arg};

use super::character::category::Category;
use super::constants::{MAX_LENGTH, MIN_LENGTH};
use super::LengthErr;

const OPTION_LENGTH: &str = "length";
const OPTION_USABLE: &str = "usable";
const OPTION_IS_EASY: &str = "is_easy";

const HELP_LENGTH: &str = "\
Length of generated password string.
Default length is 8.
Minimum length is 8.
Maximum length is 255.";

const HELP_USABLE: &str = "\
The category of characters to be used for the generated password.
    l : Lower case alphabets.
    u : Upper case alphabets.
    n : Numbers.
    s : Symbols.";

const HELP_IS_EASY: &str = "
Use easy to identify characters.
";

pub fn build() -> App<'static, 'static> {
    App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name(OPTION_LENGTH)
                .short("l")
                .long(OPTION_LENGTH)
                .takes_value(true)
                .value_name("LENGTH")
                .multiple(false)
                .validator(validate_length)
                .help(HELP_LENGTH),
        )
        .arg(
            Arg::with_name(OPTION_USABLE)
                .short("u")
                .long(OPTION_USABLE)
                .takes_value(true)
                .value_name("USABLE CHARACTER")
                .multiple(false)
                .validator(validate_usable)
                .help(HELP_USABLE),
        )
        .arg(
            Arg::with_name(OPTION_IS_EASY)
                .short("e")
                .long(OPTION_IS_EASY)
                .takes_value(false)
                .multiple(false)
                .help(HELP_IS_EASY),
        )
}

fn validate_length(value: String) -> Result<(), String> {
    let val = match value.parse::<i128>() {
        Ok(x) => x,
        Err(_) => return Err(format!("Not number value: {}", value)),
    };
    match super::validate_length(val) {
        Ok(_) => Ok(()),
        Err(e) => match e {
            LengthErr::OVER => Err(format!("Needs {} or less: {}", MAX_LENGTH, val)),
            LengthErr::UNDER => Err(format!("Needs {} or more: {}", MIN_LENGTH, val)),
        },
    }
}

fn validate_usable(value: String) -> Result<(), String> {
    match Category::validate_flag(&value) {
        Ok(_) => Ok(()),
        Err(cs) => Err(format!("unknown usable flags: {}", cs)),
    }
}

pub struct CommandLine {
    pub(crate) length: u8,
    pub(crate) usable_code: usize,
    pub(crate) is_easy: bool,
}

impl CommandLine {
    pub fn parse(app: App<'static, 'static>) -> Self {
        let arg_matches = app.get_matches();
        let length = match arg_matches.value_of(OPTION_LENGTH) {
            Some(l) => l.parse().unwrap(),
            None => 8,
        };

        let usable_code = match arg_matches.value_of(OPTION_USABLE) {
            Some(f) => Category::flags_to_code(&String::from(f)).unwrap(),
            None => Category::all_flagged_code(),
        };

        let is_easy = arg_matches.is_present(OPTION_IS_EASY);

        CommandLine {
            length: length,
            usable_code: usable_code,
            is_easy: is_easy,
        }
    }
}
