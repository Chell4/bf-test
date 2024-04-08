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

// source code (maybe with comments) to intermediate representation (operations) or error
pub fn analyze(source: String) -> Result<Vec<Operation>, FrontendError> {
    todo!()
}