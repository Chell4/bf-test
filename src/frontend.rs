pub mod lexer;
pub mod parser;

enum FrontendError {
    ParsingError(parser::ParsingError)
}

pub fn analyze(source: String) -> Result<Vec<parser::Operation>, FrontendError> {
    parser::parse(lexer::lex_analyze(lexer::pre_process(source))).map_err(FrontendError::ParsingError) // lol xD (todo)
}