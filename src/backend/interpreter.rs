use crate::frontend::Operation;

use std:: {
    collections::VecDeque,
    fmt,
    io,
    io::Read
};


type Cell = u8;

// bf runtime
struct Runtime {
    tape: Vec<Cell>,
    ptr: usize,
}

// interpreter object
pub struct Interpreter {
    runtime: Runtime,
    ops_buffer: VecDeque<Operation>
}

// error in runtime interpreter error
pub enum InterpreterError {
    EmptyBuffer,
    IOError(io::Error),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // informative debug error message
        match self {
            InterpreterError::EmptyBuffer => write!(f, "The operation buffer is empty."),
            InterpreterError::IOError(err) => write!(f, "IOError: {err}"),
        }
    }
}

impl Interpreter {
    // new interpreter constructor
    pub fn new() -> Interpreter {
        Interpreter {
            runtime: Runtime {
                tape: Vec::from([0;30000]),
                ptr: 0,
            },
            ops_buffer: VecDeque::new(),
        }
    }

    // append new operation to the buffer 
    pub fn push_op(&mut self, op: Operation) {
        self.ops_buffer.push_back(op);
    }

    // append operations to the buffer
    pub fn push_ops(&mut self, ops: Vec<Operation>) {
        for i in ops {
            self.ops_buffer.push_back(i);
        }
    }

    // execute and pop the first operation in the buffer
    pub fn execute_next(&mut self) -> Option<InterpreterError> {
        match self.ops_buffer.pop_front() {
            None => Some(InterpreterError::EmptyBuffer),
            Some(op) => match op {
                Operation::Add => {
                    self.runtime.tape[self.runtime.ptr] += 1;
                    None
                },
                Operation::Sub => {
                    self.runtime.tape[self.runtime.ptr] -= 1;
                    None
                },
                Operation::MoveLeft => {
                    self.runtime.ptr -= 1;
                    None
                },
                Operation::MoveRight => {
                    self.runtime.ptr += 1;
                    None
                },
                Operation::Input => {
                    let mut buf = Vec::new();
                    match io::stdin().read_exact(&mut buf){
                        Err(err) => return Some(InterpreterError::IOError(err)),
                        Ok(_) => (),
                    }
                    self.runtime.tape[self.runtime.ptr] = buf[0];
                    None
                },
                Operation::Print => {
                    print!("{}", self.runtime.tape[self.runtime.ptr] as char);
                    None
                },
                Operation::Loop(vec_ops) => {
                    let mut interpreter = Interpreter::new();
                    let mut runtime_buf = Runtime{
                        tape: Vec::new(),
                        ptr: 0,
                    };
                    std::mem::swap(&mut runtime_buf, &mut self.runtime);
                    interpreter.runtime = runtime_buf;
                    
                    while interpreter.runtime.tape[interpreter.runtime.ptr] != 0 {
                        interpreter.push_ops(vec_ops.clone());
                        if let Some(err) = interpreter.execute_all() {
                            return Some(err)
                        }
                    }
                    self.runtime = interpreter.runtime;
                    None
                },
                Operation::NoOp => None,
            },
        }
    }

    // execute all operations and clear the buffer
    pub fn execute_all(&mut self) -> Option<InterpreterError> {
        while self.buffer_size() > 0 {
            if let Some(err) = self.execute_next() {
                return Some(err)
            }
        }
        None
    }

    // pop last n (or less if not possible) pushed operations
    pub fn pop_buffer(&mut self, n: usize) {
        {
            if self.ops_buffer.len() <= n {
                self.ops_buffer.clear();
            } else {
                for _ in 0..n {
                    self.ops_buffer.pop_back();
                }
            }
        }
    }

    // return number of buffered operations
    pub fn buffer_size(&self) -> usize {
        self.ops_buffer.len()
    }
}


// just interpret given vector of operations
pub fn interpret(ops: &Vec<Operation>) -> Option<InterpreterError> {
    let mut interpreter = Interpreter::new();
    interpreter.push_ops(ops.clone());
    interpreter.execute_all()
}

