use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

use crate::errors::ParseTokenError;

#[derive(Debug, PartialEq)]
pub enum Token {
    OOH,
    EEE,
    AAH,
}

impl Token {
    const TOKEN_SIZE: usize = 3;
}

impl FromStr for Token {
    type Err = ParseTokenError;
    fn from_str(input: &str) -> Result<Token, Self::Err> {
        match input {
            "ooh" => Ok(Token::OOH),
            "eee" => Ok(Token::EEE),
            "aah" => Ok(Token::AAH),
            _ => Err(ParseTokenError::new(input)),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(input: &str) -> Result<Parser, ParseTokenError> {
        let mut p = Parser { tokens: Vec::new() };
        let tokenize_result = p.tokenize(input);
        match tokenize_result {
            Ok(_) => Ok(p),
            Err(err) => Err(err),
        }
    }

    fn tokenize(&mut self, s: &str) -> Result<(), ParseTokenError> {
        let valid_chars: HashSet<char> = vec!['a', 'o', 'h', 'e'].into_iter().collect();
        let mut chars: Vec<char> = Vec::with_capacity(4);
        for c in s.chars() {
            // Ignore invalid characters
            if valid_chars.contains(&c) {
                chars.push(c);
                if chars.len() == Token::TOKEN_SIZE {
                    let potential_token = String::from_iter(chars.clone());
                    chars.clear();
                    let res = Token::from_str(&potential_token);
                    match res {
                        Ok(token) => self.tokens.push(token),
                        Err(error) => return Err(error),
                    }
                }
            }
        }
        // If our char vector is non empty, that means we have some hanging letters
        if !chars.is_empty() {
            let invalid_token = String::from_iter(chars);
            panic!(
                "Unable to parse string, {}",
                ParseTokenError::new(&invalid_token.to_string())
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_success() {
        let test_string = "ooh oohaah aaheeeeee";
        let expected_tokens = vec![
            Token::OOH,
            Token::OOH,
            Token::AAH,
            Token::AAH,
            Token::EEE,
            Token::EEE,
        ];
        let parser = Parser::new(test_string).expect("should be able to parse this");
        assert_eq!(expected_tokens, parser.tokens);
    }

    #[test]
    fn test_parser_failure() {
        let test_string = "ooh ooh aah aaheee eeeoho";
        let parser = Parser::new(test_string).expect_err("should fail to parse this");
        assert_eq!(ParseTokenError::new("oho"), parser)
    }
}
