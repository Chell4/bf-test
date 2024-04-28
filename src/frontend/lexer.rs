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
pub struct Lexer {
    str_buffer: VecDeque<String>
}

pub enum LexerError {
    LexerIsEmpty: "Can't analyze and clear empty buffer"

}

impl Lexer {
    // create new lexer
    pub fn new() -> Lexer {
        Lexer{
            str_buffer: VecDeque::new(),
        }
    }

    // clear lexer buffer
    pub fn clear(&mut self) {
        self.Lexer.str_buffer.clear();
    }

    // push string to the buffer
    pub fn push_str(&mut self, str: String) {
        self.Lexer.str_buffer.push_back(str);
    }
    // pop last pushed string
    pub fn pop_str(&mut self) {
        self.Lexer.pop_back();
    }

    // analyze and pop oldest pushed string
    pub fn analyze_next(&mut self) -> Result<Vec<Token>, LexerError> {
        
    }
    // analyze all buffered strings and clear the buffer
    pub fn analyze_all (&mut self) -> Result<Vec<Token>, LexerError> {
        let mut vec = Vec::new();
        if !self.Lexer.size(){
            for i in self {
                Vec.push_back(self[i]);
            }
        } else {
            return LexerIsEmpty;
        }
        self.Lexer.clear();
    }

    // returns size of the character buffer
    pub fn buffer_size(&self) -> usize {
        self.Lexer.size()
    }
}

// removes all characters except allowed
pub fn pre_process(source: String) -> String {
    for i in source {
        if ((source[i] != "+") & (source[i] != "-") & (source[i] != ">") & (source[i] != "<") & (source[i] != "[") & (source[i] != "]") & (source[i] != ".") & (source[i] != ",")) {
            source.pop(source[i]);
        } 
    }
}
