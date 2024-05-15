use std::fmt;

mod lexer;
mod parser;

pub use lexer::{
    Token,
    Lexer,
    LexerError,
    pre_process,
};
pub use parser::{
    Operation,
    Parser,
    ParsingError,
};

// any error from frontend
pub enum FrontendError {
    ParsingError(ParsingError),
    LexerError(LexerError),
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrontendError::LexerError(err) => write!(f, "lexical analysis failed: {err}"),
            FrontendError::ParsingError(err) => write!(f, "syntactic analysis failed: {err}")
        }
    }
}

// source code (maybe with comments) to intermediate representation (operations) or error
pub fn analyze(mut source: String) -> Result<Vec<Operation>, FrontendError> {
    let mut lexer = Lexer::new();

    source = pre_process(source);

    lexer.push_str(source);
    
    let mut parser = Parser::new();
    match lexer.analyze_all() {
        Err(err) => return Err(FrontendError::LexerError(err)),
        Ok(tokens) => parser.push_tokens(&tokens)
    }
    
    match parser.parse_all() {
        Err(err) => Err(FrontendError::ParsingError(err)),
        Ok(ops) => Ok(ops)
    }
}