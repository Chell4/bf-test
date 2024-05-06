    use crate::frontend;
    use crate::frontend::Token;

    use std::{
        collections::VecDeque,
        fmt,
    };

    // bf operation
    pub enum Operation {
        NoOp,
        Add,
        Sub,
        MoveLeft,
        MoveRight,
        Input,
        Print,
        Loop(Vec<Operation>)
    }

    // parser
    pub struct Parser {
        token_buffer: VecDeque<Token>
    }

    // parser error
    pub enum ParsingError {
        ParserIsEmpty
    }

    impl fmt::Debug for ParsingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // informative debug error message
            match fmt::Debug {
                ParsingError::ParserIsEmpty => "You trying to interact with empty buffer",
            }
        }
    }

    impl Parser {
        // new parser
        pub fn new() -> Parser {
            Parser{
                token_buffer: VecDeque::new(),
            }
        }

        // clear parser buffer
        pub fn clear(&mut self) {
            self.token_buffer.clear();
        }

        // push single token
        pub fn push_token(&mut self, token: &Token) {
            self.token_buffer.push_back(*token);
        }

        // push the slice of the tokens
        pub fn push_tokens(&mut self, tokens: &[Token]) {
            for i in tokens{
                self.token_buffer.push_back(*i);
            }
        }

        // try to parse operation from oldest tokens
        pub fn parse_next_op(&mut self) -> Result<Operation, ParsingError> {
            let mut vec = new::Vec;
            match self.token_buffer.back() {
                None => Err(ParsingError::ParserIsEmpty),
                Some(s) => {
                    match s {
                        Token::Plus => Ok(Operation::Add),
                        Token::Minus => Ok(Operation::Sub),
                        Token::LeftArrow => Ok(Operation::MoveLeft),
                        Token::RightArrow => Ok(Operation::MoveRight),
                        Token::Point => Ok(Operation::Input),
                        Token::Comma => Ok(Operation::Print),
                        Token::OpenBracket => for i in self.token_buffer  {
                            vec.push(parse_next_op());

                        },
                        Token::CloseBracket => ,
                    }
                }
            }
        }
        // try to parse entire buffer or return first error
        pub fn parse_all(&mut self) -> Result<Vec<Operation>, ParsingError> {
            let mut vec = new::Vec();
            for i in self.token_buffer {
                vec.push(parse_next_op());
            }
            Ok(vec)
        }
        // parse until the predicate return true
        pub fn parse_until(&mut self, mut f: impl FnMut(&Token) -> bool) -> Result<Vec<Operation>, ParsingError> {
            let mut vec = new::Vec();
            while FnMut {
                vec.push(parse_next_op());
            }
            Ok(vec)
        }

        // return number of tokens in the buffer
        pub fn buffer_size(&self) -> usize {
            self.token_buffer.len()
        }
    }
