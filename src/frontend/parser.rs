use super::lexer;

use std::fmt;

pub enum Operation {
    Add,
    Sub,
    MoveLeft,
    MoveRight,
    Input,
    Print,
    Loop(Vec<Operation>)
}

pub enum ParsingError {
    //todo
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!() // informative debug error message
    }
}

pub fn parse(tokens: Vec<lexer::Token>) -> Result<Vec<Operation>, ParsingError> {
    todo!()
}