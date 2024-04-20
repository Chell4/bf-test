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

pub enum LexerError {
    // todo

}

impl<'a> Lexer<'a> {
    // create new lexer
    pub fn new() -> Lexer<'a> {
        todo!()
    }

    // clear lexer buffer
    pub fn clear(&mut self) {
        self.clear();  //11.04
    }

    // push string to the buffer
    pub fn push_str(&mut self, str: &'a str) {
        self.push_back(str); //11.04
    }
    // pop last pushed string
    pub fn pop_str(&mut self) {
        self.pop_back(); //11.04
    }

    // analyze and pop oldest pushed string
    pub fn analyze_next(&mut self) -> Result<Vec<Token>, LexerError> {
        for i in self {
            vec.push_back(self[i]);
        }
        self.pop(self[0]);
    }
    // analyze all buffered strings and clear the buffer
    pub fn analyze_all (&mut self) -> Result<Vec<Token>, LexerError> {
        for i in self {
            vec.push_back(self[i]);
        }
        self.clear();
    }

    // returns size of the character buffer
    pub fn buffer_size(&self) -> usize {
        self.size()
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
