mod lexer;
mod parser;

pub use lexer::{
    Token,
    pre_process,
    lex_analyze,
};
pub use parser::{
    Operation,
    ParsingError,
    parse,
};

pub enum FrontendError {
    ParsingError(ParsingError)
}

pub fn analyze(source: String) -> Result<Vec<Operation>, FrontendError> {
    parse(lex_analyze(pre_process(source))).map_err(FrontendError::ParsingError) // lol xD (todo)
}