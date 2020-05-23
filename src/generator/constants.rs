pub(crate) const LOWER_CHARS_ALL: &str = "abcdefghijklmnopqrstuvwxyz";
pub(crate) const UPPER_CHARS_ALL: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub(crate) const NUMBER_CHARS_ALL: &str = "0123456789";
pub(crate) const SYMBOL_CHARS_ALL: &str = r##"!@#$%^&*()\=+_-{}[]:`~|'"<>?;/.,"##;

pub(crate) const LOWER_CHARS_EASY: &str = "abcdefghijkmnpqrstuvwxyz";
pub(crate) const UPPER_CHARS_EASY: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
pub(crate) const NUMBER_CHARS_EASY: &str = "23456789";
pub(crate) const SYMBOL_CHARS_EASY: &str = r##"!@#$%^&*=+~"<>?"##;

pub(crate) const LOWER_FLAG: &str  = "l";
pub(crate) const UPPER_FLAG: &str  = "u";
pub(crate) const NUMBER_FLAG: &str  = "n";
pub(crate) const SYMBOL_FLAG: &str  = "s";

pub(crate) const MIN_LENGTH :i128 = 8;
pub(crate) const MAX_LENGTH :i128 = std::u8::MAX as i128;
