
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
    //todo
}

impl Interpreter {
    // new interpreter constructor
    pub fn new() -> Interpreter {
        todo!()
    }
    
    // append new operation to the buffer 
    pub fn push_op(&mut self, op: &Operation) {
        todo!()
    }
    // append operations to the buffer
    pub fn push_ops(&mut self, ops: &Vec<Operation>) {
        todo!()
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
        todo!()
    }
    
    // return number of buffered operations
    pub fn buffer_size(&self) -> usize {
        todo!()
    }
}

// just interpret given vector of operations
pub fn interpret(ops: Vec<Operation>) -> Option<InterpreterError> {
    todo!()
}