use crate::frontend::Token;

use std::{
    collections::VecDeque,
    fmt,
};

#[derive(Clone)]

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
    ParserIsEmpty,
    MismatchedBracket,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // informative debug error message
        match self {
            ParsingError::ParserIsEmpty => write!(f, "The parser buffer is empty."),
            ParsingError::MismatchedBracket => write!(f, "Cannot find a matching bracket."),
        }
    }
}

impl Parser {
    // new parser
    pub fn new() -> Parser {
        Parser{
            token_buffer: VecDeque::new(),
        }
    }

    // clear parser buffer
    pub fn clear(&mut self) {
        self.token_buffer.clear();
    }

    // push single token
    pub fn push_token(&mut self, token: &Token) {
        self.token_buffer.push_back(*token);
    }

    // push the slice of the tokens
    pub fn push_tokens(&mut self, tokens: &[Token]) {
        for i in tokens{
            self.token_buffer.push_back(*i);
        }
    }

    // try to parse operation from oldest tokens
    pub fn parse_next_op(&mut self) -> Result<Operation, ParsingError> {
        let mut flag = false;
        match self.parse_until(|_| {
            let tmp = flag;
            flag = true;
            tmp
        }) {
            Ok(vec_ops) => Ok(vec_ops[0].clone()),
            Err(err) => Err(err),
        }
    }

    // try to parse entire buffer or return first error
    pub fn parse_all(&mut self) -> Result<Vec<Operation>, ParsingError> {
        self.parse_until(|_| false)
    }

    // parse until the predicate return true
    pub fn parse_until(&mut self, mut f: impl FnMut(&Token) -> bool) -> Result<Vec<Operation>, ParsingError> {
        let mut vec = Vec::new();
        while let Some(token) = self.token_buffer.pop_front() {
            if f(&token) {
                self.token_buffer.push_front(token);
                return Ok(vec);
            }
            match token {
                Token::Plus => vec.push(Operation::Add),
                Token::Minus => vec.push(Operation::Sub),
                Token::LeftArrow => vec.push(Operation::MoveLeft),
                Token::RightArrow => vec.push(Operation::MoveRight),
                Token::Point => vec.push(Operation::Input),
                Token::Comma => vec.push(Operation::Print),
                Token::OpenBracket => match self.parse_until(|t| *t == Token::CloseBracket) {
                    Ok(loop_ops) => {
                        if self.token_buffer.pop_front() != Some(Token::CloseBracket) {
                            return Err(ParsingError::MismatchedBracket);
                        }
                        vec.push(Operation::Loop(loop_ops));
                        
                    },
                    Err(err) => return Err(err),
                },
                Token::CloseBracket => return Err(ParsingError::MismatchedBracket),
            }
        }
        Ok(vec)
    }


    // return number of tokens in the buffer
    pub fn buffer_size(&self) -> usize {
        self.token_buffer.len()
    }
}