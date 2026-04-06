use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Missing required field: {0}")]
    MissingRequired(&'static str),

    #[error("Invalid value for '{field}': expected {expected}, got '{got}'")]
    InvalidValue {
        field: &'static str,
        expected: &'static str,
        got: String,
    },

    #[error("Failed to parse HTML document")]
    HtmlParse,
}
