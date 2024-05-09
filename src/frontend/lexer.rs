use std::collections::VecDeque;

#[derive(Clone)]
#[derive(Copy)]
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
    ExpectedOpenBracket,
    ExpectedClosedBracket,
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
        let mut res_vec = Vec::new();
        match self.str_buffer.pop_front() {
            None => Err(LexerError::LexerIsEmpty),
            Some(s) => {
                for i in s.chars().map(|c| Token::from(c)) {
                    match i {
                        Some(token) => res_vec.push(token),
                        None => return Err(LexerError::WrongCharacter),
                    }
                }
                Ok(res_vec)
            },
        }
    }

    // analyze all buffered strings and clear the buffer
    pub fn analyze_all (&mut self) -> Result<Vec<Token>, LexerError> {
        todo!();
    }

    // returns size of the character buffer
    pub fn buffer_size(&self) -> usize {
        self.str_buffer.len()
    }
}


// removes all characters except allowed
pub fn pre_process(source: String) -> String {
    let parentheses_checker: VecDeque<char> = VecDeque::new();
    source.chars()
        .filter(|c| String::from("+-<>[].,").contains(*c))
        .collect(),

    for i in source {
        match i {
            '[' => parentheses_checker.push_front(i),
            ']' => match parentheses_checker.pop_front() {
                None => Some(LexerError::ExpectedOpenBracket),
                Some => None,
            }
        }
    }

    match parentheses_checker.pop_front() {
        None => None,
        Some => Some(LexerError::ExpectedClosedBracket),
    }
}
