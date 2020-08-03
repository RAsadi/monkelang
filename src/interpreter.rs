use std::io::{self, Read};

use crate::parser;

pub struct Interpreter {
    memory: [u8; 30000],
    curr_cell: usize,
    pc: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            memory: [0; 30000],
            curr_cell: 0,
            pc: 0,
        }
    }

    pub fn interpret(&mut self, tokens: Vec<parser::Token>) {
        let mut user_input = String::new();
        while self.pc < tokens.len() {
            let curr_token = &tokens[self.pc];
            match curr_token {
                parser::Token::OOH_OOH => self.curr_cell += 1,
                parser::Token::OOH_EEE => self.curr_cell -= 1,
                parser::Token::OOH_AAH => self.memory[self.curr_cell] += 1,
                parser::Token::EEE_EEE => self.memory[self.curr_cell] -= 1,
                parser::Token::EEE_AAH => print!("{}", self.memory[self.curr_cell] as char),
                parser::Token::AAH_OOH => {
                    io::stdin()
                        .read_to_string(&mut user_input)
                        .expect("Failed to read user input");
                    self.memory[self.curr_cell] = user_input.as_bytes()[0];
                }
                parser::Token::AAH_EEE => {
                    // If the memory at the curr cell is zero, we need to break out of the loop,
                    // so we find the corresponding closing aah aah by looking ahead.
                    if self.memory[self.curr_cell] == 0 {
                        let mut nest_level = 0;
                        loop {
                            self.pc += 1;
                            if tokens[self.pc] == parser::Token::AAH_EEE {
                                // hit a nested loop
                                nest_level += 1;
                                continue;
                            } else if nest_level > 0 && tokens[self.pc] == parser::Token::AAH_AAH {
                                nest_level -= 1;
                                continue;
                            } else if nest_level == 0 && tokens[self.pc] == parser::Token::AAH_AAH {
                                break;
                            }
                            if self.pc > tokens.len() {
                                panic!("Invalid code, no closing aah aah")
                            }
                        }
                    }
                }
                parser::Token::AAH_AAH => {
                    // if the memory here is non zero, we need to backtrack until we find the
                    // corresponding starting aah eee to run the loop again
                    if self.memory[self.curr_cell] != 0 {
                        let mut nest_level = 0;
                        loop {
                            self.pc -= 1;
                            if self.pc == tokens.len() - 1 || self.pc == usize::MAX {
                                panic!("Invalid code, no closing aah eee")
                            }
                            if tokens[self.pc] == parser::Token::AAH_AAH {
                                // hit a nested loop
                                nest_level += 1;
                                continue;
                            } else if nest_level > 0 && tokens[self.pc] == parser::Token::AAH_EEE {
                                nest_level -= 1;
                                continue;
                            } else if nest_level == 0 && tokens[self.pc] == parser::Token::AAH_EEE {
                                break;
                            }
                        }
                    }
                }
            }
            self.pc += 1;
        }
    }
}
