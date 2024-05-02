
use crate::frontend::Operation;

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
    
}

impl Interpreter {
    // new interpreter constructor
    pub fn new() -> Interpreter {
        Interpreter {
            runtime: Runtime{
                tape: Vec<Cell>::new(),
                ptr: usize::new(),
            }
            ops_buffer: VecDeque<Operation>::new(),
        }
    }
    
    // append new operation to the buffer 
    pub fn push_op(&mut self, op: &Operation) {
        self.ops_buffer.push_back(&op);
    }
    // append operations to the buffer
    pub fn push_ops(&mut self, ops: &Vec<Operation>) {
        for i in ops {
            self.ops_buffer.push_back(ops[i]);
        }
    }
    
    // execute and pop the first operation in the buffer
    pub fn execute_next(&mut self) -> Option<InterpreterError> {
        todo!()
    }
    // execute all operations and clear the buffer
    pub fn execute_all(&mut self) -> Option<InterpreterError> {
        todo!()
    }

    // pop last n (or less if not possible) pushed operations
    pub fn pop_buffer(&mut self, n: usize) {
        if (self.ops_buffer.len() <= n) {
            self.ops_buffer.clear();
        } else {
            for i in &self.ops_buffer {
                self.ops_buffer.pop_back();
            }
        }
    }
    
    // return number of buffered operations
    pub fn buffer_size(&self) -> usize {
        self.ops_buffer.len()
    }
}

// just interpret given vector of operations
pub fn interpret(ops: Vec<Operation>) -> Option<InterpreterError> {
    
    for i in &ops {
        match ops[i] {
            NoOp => ;
            Add => self.runtime.tape[ptr] += 1;
            Sub => self.runtime.tape[ptr] += 1;
            MoveLeft => ptr -= 1;
            MoveRight => ptr += 1;
            Input => ;
            Print => ;
            Loop => ;
        }
    }
}
