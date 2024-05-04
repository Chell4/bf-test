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

impl Token {
    pub fn from(c: char) -> Option<Self> {
        match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '[' => Some(Token::OpenBracket),
            ']' => Some(Token::CloseBracket),
            '.' => Some(Token::Point),
            ',' => Some(Token::Comma),
            '<' => Some(Token::LeftArrow),
            '>' => Some(Token::RightArrow),

            _ => None,
        }
    }
}

// lexical analyzer
pub struct Lexer {
    str_buffer: VecDeque<String>
}

pub enum LexerError {
    LexerIsEmpty,
    WrongCharacter,
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
        self.str_buffer.clear();
    }

    // push string to the buffer
    pub fn push_str(&mut self, str: String) {
        self.str_buffer.push_back(str);
    }
    // pop last pushed string
    pub fn pop_str(&mut self) {
        self.str_buffer.pop_back();
    }

    // analyze and pop oldest pushed string
    pub fn analyze_next(&mut self) -> Result<Vec<Token>, LexerError> {
        match self.str_buffer.pop_front() {
            None => Err(LexerError::LexerIsEmpty),
            Some(s) => {
                Ok(s.chars().map(|c| String::from("+-<>[].,").contains(*c)))
            },
        }
    }

    // analyze all buffered strings and clear the buffer
    pub fn analyze_all (&mut self) -> Result<Vec<Token>, LexerError> {
        for i in self.str_buffer {
            self.analyze_next(i)
        }
    }

    // returns size of the character buffer
    pub fn buffer_size(&self) -> usize {
        self.str_buffer.len()
    }
}


// removes all characters except allowed
pub fn pre_process(source: String) -> String {
    source.chars()
        .filter(|c| String::from("+-<>[].,").contains(*c))
        .collect()
}
