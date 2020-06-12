use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Error {
    LengthInsufficientErr(i128),
    LengthExcessErr(i128),
    CategoryFlagErr(String),
    NotSymbolErr(String),
    CharactersErr(()),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::LengthInsufficientErr(i) | Error::LengthExcessErr(i) => i.fmt(f),
            Error::CategoryFlagErr(e) | Error::NotSymbolErr(e) => e.fmt(f),
            Error::CharactersErr(_) => "Usable character category is not found".fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::LengthInsufficientErr(_)
            | Error::LengthExcessErr(_)
            | Error::CategoryFlagErr(_)
            | Error::NotSymbolErr(_)
            | Error::CharactersErr(_) => None,
        }
    }
}
