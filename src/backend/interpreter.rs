use crate::{backend::interpreter, frontend::Operation};

use std::collections::VecDeque;


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
    InfiniteLoop,
}

impl Interpreter {
    // new interpreter constructor
    pub fn new() -> Interpreter {
        Interpreter {
            runtime: Runtime {
                tape: Vec::new(),
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
        let input;
        match self.ops_buffer.pop_front() {
            Operation::Add => self.runtime.tape[self.runtime.ptr] += 1,
            Operation::Sub => self.runtime.tape[self.runtime.ptr] -= 1,
            Operation::MoveLeft => self.runtime.ptr -= 1,
            Operation::MoveRight => self.runtime.ptr += 1,
            Operation::Input => self.runtime.tape[ptr] = input,
            Operation::Print => println!(self.runtime.tape[ptr]),
            Operation::Loop => ,
        }
    }
    // execute all operations and clear the buffer
    pub fn execute_all(&mut self) -> Option<InterpreterError> {
        for i in self.ops_buffer() {
            execute_next();
        }
    }

    // pop last n (or less if not possible) pushed operations
    pub fn pop_buffer(&mut self, n: usize) {
        {
            if (self.ops_buffer.len() <= n) {
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
    let interpreter = Interpreter::new();
    push_ops(ops);
    execute_all();
}
