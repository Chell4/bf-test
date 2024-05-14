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
    match lexer::Lexer::analyze_all(lexer::Lexer::push_str(lexer::Lexer::pre_process(source))) {
        Err(err) => Err(err),
        Ok(vt) => match parser::Parser::parse_all(parser::Parser::push_tokens(vt)) {
            Err(er) => Err(er),
            Ok(an) => Ok(an),
        },
    }
    
}