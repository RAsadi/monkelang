use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

use crate::errors::ParseTokenError;
use crate::errors::TranslationError;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    OOH_OOH,
    OOH_EEE,
    OOH_AAH,
    EEE_EEE,
    EEE_AAH,
    AAH_OOH,
    AAH_EEE,
    AAH_AAH,
}

impl Token {
    pub fn from_units(tok1: &UnitToken, tok2: &UnitToken) -> Result<Token, ParseTokenError> {
        if tok1 == &UnitToken::OOH && tok2 == &UnitToken::OOH {
            return Ok(Token::OOH_OOH);
        } else if tok1 == &UnitToken::OOH && tok2 == &UnitToken::EEE {
            return Ok(Token::OOH_EEE);
        } else if tok1 == &UnitToken::OOH && tok2 == &UnitToken::AAH {
            return Ok(Token::OOH_AAH);
        } else if tok1 == &UnitToken::EEE && tok2 == &UnitToken::EEE {
            return Ok(Token::EEE_EEE);
        } else if tok1 == &UnitToken::EEE && tok2 == &UnitToken::AAH {
            return Ok(Token::EEE_AAH);
        } else if tok1 == &UnitToken::AAH && tok2 == &UnitToken::OOH {
            return Ok(Token::AAH_OOH);
        } else if tok1 == &UnitToken::AAH && tok2 == &UnitToken::EEE {
            return Ok(Token::AAH_EEE);
        } else if tok1 == &UnitToken::AAH && tok2 == &UnitToken::AAH {
            return Ok(Token::AAH_AAH);
        } else {
            Err(ParseTokenError::new(&format!("{:?}, {:?}", tok1, tok2).to_string()))
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token_text = match self {
            Token::OOH_OOH => "ooh ooh",
            Token::OOH_EEE => "ooh eee",
            Token::OOH_AAH => "ooh aah",
            Token::EEE_EEE => "eee eee",
            Token::EEE_AAH => "eee aah",
            Token::AAH_OOH => "aah ooh",
            Token::AAH_EEE => "aah eee",
            Token::AAH_AAH => "aah aah",
        };
        write!(f, "{}", token_text)
    }
}
#[derive(Debug, PartialEq)]
pub enum UnitToken {
    OOH,
    EEE,
    AAH,
}

impl UnitToken {
    const TOKEN_SIZE: usize = 3;
}

impl FromStr for UnitToken {
    type Err = ParseTokenError;
    fn from_str(input: &str) -> Result<UnitToken, Self::Err> {
        match input {
            "ooh" => Ok(UnitToken::OOH),
            "eee" => Ok(UnitToken::EEE),
            "aah" => Ok(UnitToken::AAH),
            _ => Err(ParseTokenError::new(input)),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        return Parser { tokens: Vec::new() };
    }

    pub fn tokenize(s: &str) -> Result<Parser, ParseTokenError> {
        let mut parser = Parser { tokens: Vec::new() };
        let valid_chars: HashSet<char> = vec!['a', 'o', 'h', 'e'].into_iter().collect();
        let mut chars: Vec<char> = Vec::with_capacity(4);
        let mut units: Vec<UnitToken> = Vec::new();
        for c in s.chars() {
            // Ignore invalid characters
            if valid_chars.contains(&c) {
                chars.push(c);
                if chars.len() == UnitToken::TOKEN_SIZE {
                    let potential_token = String::from_iter(chars.clone());
                    chars.clear();
                    let res = UnitToken::from_str(&potential_token);
                    match res {
                        Ok(token) => units.push(token),
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

        // Process the units into pairs
        for (i, unit) in units.iter().enumerate() {
            if i % 2 == 1 {
                let new_token = Token::from_units(&units[i - 1], unit);
                match new_token {
                    Ok(token) => parser.tokens.push(token),
                    Err(error) => return Err(error),
                }
            }
        }

        // Hanging pairs are invalid
        if units.len() % 2 != 0 {
            // Guaranteed unwrap since %2 != 0
            return Err(ParseTokenError::new(
                &format!("pair<{:?}, ?>", units.last().unwrap()).to_string(),
            ));
        }
        Ok(parser)
    }

    fn to_bf(&self) -> String {
        let mut output = String::new();
        for token in &self.tokens {
            let bf_char = match token {
                Token::OOH_OOH => '>',
                Token::OOH_EEE => '<',
                Token::OOH_AAH => '+',
                Token::EEE_EEE => '-',
                Token::EEE_AAH => '.',
                Token::AAH_OOH => ',',
                Token::AAH_EEE => '[',
                Token::AAH_AAH => ']',
            };
            output.push(bf_char);
        }
        return output;
    }

    fn from_bf(&mut self, bf: &str) -> Result<(), TranslationError> {
        self.tokens.clear();
        for c in bf.chars() {
            let monke_char = match c {
                '>' => Token::OOH_OOH,
                '<' => Token::OOH_EEE,
                '+' => Token::OOH_AAH,
                '-' => Token::EEE_EEE,
                '.' => Token::EEE_AAH,
                ',' => Token::AAH_OOH,
                '[' => Token::AAH_EEE,
                ']' => Token::AAH_AAH,
                _ => return Err(TranslationError::new(&c.to_string())),
            };
            self.tokens.push(monke_char);
        }
        Ok(())
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

impl fmt::Display for Parser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut self_str = String::new();
        for token in &self.tokens {
            self_str.push_str(&format!("{} ", token));
        }
        write!(f, "{}", self_str)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser_success() {
        let test_string = "ooh oohaah aaheeeeee";
        let expected_tokens = vec![Token::OOH_OOH, Token::AAH_AAH, Token::EEE_EEE];
        let parser = Parser::tokenize(test_string).expect("should be able to parse this");
        assert_eq!(expected_tokens, parser.tokens);
    }

    #[test]
    fn test_parser_success_pairs() {
        let test_string = "ooh aah aah eee eee aah";
        let expected_tokens = vec![Token::OOH_AAH, Token::AAH_EEE, Token::EEE_AAH];
        let parser = Parser::tokenize(test_string).expect("should be able to parse this");
        assert_eq!(expected_tokens, parser.tokens);
    }

    #[test]
    fn test_parser_failure() {
        let test_string = "ooh ooh aah aaheee eeeoho";
        let parser = Parser::tokenize(test_string).expect_err("should fail to parse this");
        assert_eq!(ParseTokenError::new("oho"), parser)
    }

    #[test]
    fn test_parser_invalid_pair_failure() {
        let test_string = "ooh ooh aah aah eee ooh";
        let parser = Parser::tokenize(test_string).expect_err("should fail to parse this");
        assert_eq!(ParseTokenError::new("EEE, OOH"), parser)
    }

    #[test]
    fn test_parser_hanging_single() {
        let test_string = "ooh";
        let parser = Parser::tokenize(test_string).expect_err("should fail to parse this");
        assert_eq!(ParseTokenError::new("pair<OOH, ?>"), parser)
    }

    #[test]
    fn test_parser_bf_hello_world() {
        let hello_world_bf = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>-----.++++++[->++<]>+.[--->+<]>.-.---.------.[--->+<]>++.";
        let mut parser = Parser::new();
        parser.from_bf(hello_world_bf).expect("should not fail to translate");
        println!("Hello world: {}", parser);
        assert_eq!(hello_world_bf, parser.to_bf());
    }
}
