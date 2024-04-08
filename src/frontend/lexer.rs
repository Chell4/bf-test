use std::collections::VecDeque;

pub enum Token {
    Plus,
    Minus,
    LeftArrow,
    RightArrow,
    Point,
    Comma,
    OpenBracket,
    CloseBracket,
}

// lexical analyzer
pub struct Lexer<'a> {
    str_buffer: VecDeque<&'a str>
}

pub struct LexerError {
    // todo
}

impl<'a> Lexer<'a> {
    // create new lexer
    pub fn new() -> Lexer<'a> {
        todo!()
    }
    
    // clear lexer buffer
    pub fn clear(&mut self) {
        todo!()
    }
    
    // push string to the buffer
    pub fn push_str(&mut self, str: &'a str) {
        todo!()
    }
    // pop last pushed string
    pub fn pop_str(&mut self) {
        todo!()
    }
    
    // analyze and pop oldest pushed string
    pub fn analyze_next(&mut self) -> Result<Vec<Token>, LexerError> {
        todo!()
    }
    // analyze all buffered strings and clear the buffer
    pub fn analyze_all (&mut self) -> Result<Vec<Token>, LexerError> {
        todo!()
    }
    
    // returns size of the character buffer
    pub fn buffer_size(&self) -> usize {
        todo!()
    }
}

// removes all characters except allowed
pub fn pre_process(source: String) -> String {
    todo!() 
}