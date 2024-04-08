use crate::frontend;
use crate::frontend::Token;

use std::{
    collections::VecDeque,
    fmt,
};

// bf operation
pub enum Operation {
    NoOp,
    Add,
    Sub,
    MoveLeft,
    MoveRight,
    Input,
    Print,
    Loop(Vec<Operation>)
}

// parser
pub struct Parser {
    token_buffer: VecDeque<Token>
}

// parser error
pub enum ParsingError {
    //todo
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!() // informative debug error message
    }
}

impl Parser {
    // new parser
    pub fn new() -> Parser {
        todo!()
    }
    
    // clear parser buffer
    pub fn clear(&mut self) {
        todo!()
    }
    
    // push single token
    pub fn push_token(&mut self, token: &Token) {
        todo!()
    }
    // push the slice of the tokens 
    pub fn push_tokens(&mut self, tokens: &[Token]) {
        todo!()
    }
    
    // try to parse operation from oldest tokens
    pub fn parse_next_op(&mut self) -> Result<Operation, ParsingError> {
        todo!()
    }
    // try to parse entire buffer or return first error
    pub fn parse_all(&mut self) -> Result<Vec<Operation>, ParsingError> {
        todo!()
    }
    // parse until the predicate return true
    pub fn parse_until(&mut self, mut f: impl FnMut(&Token) -> bool) -> Result<Vec<Operation>, ParsingError> {
        todo!()
    }

    // return number of tokens in the buffer
    pub fn buffer_size(&self) -> usize {
        todo!()
    }
}