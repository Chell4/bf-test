
pub enum Token {
    Plus,
    Minus,
    LeftArrow,
    RightArrow,
    Point,
    Comma,
    OpenBracket,
    CloseBracket,
    Unknown,
}

// removes all characters except allowed
pub fn pre_process(source: String) -> String {
    todo!() 
}

// maps all characters of the source string to the corresponding tokens (or Unknown)
pub fn lex_analyze(source: String) -> Vec<Token> {
    todo!()
}