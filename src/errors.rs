use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ParseTokenError {
    token: String,
}

impl ParseTokenError {
    pub fn new(msg: &str) -> ParseTokenError {
        ParseTokenError { token: msg.to_string() }
    }
}

impl fmt::Display for ParseTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid token/token_pair: {}", self.token)
    }
}

impl Error for ParseTokenError {
    fn description(&self) -> &str {
        &self.token
    }
}

#[derive(Debug, PartialEq)]
pub struct TranslationError {
    bf_token: String,
}

impl TranslationError {
    pub fn new(msg: &str) -> TranslationError {
        TranslationError {
            bf_token: msg.to_string(),
        }
    }
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid bf token: {}", self.bf_token)
    }
}

impl Error for TranslationError {
    fn description(&self) -> &str {
        &self.bf_token
    }
}
