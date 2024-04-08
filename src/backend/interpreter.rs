
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

// error in runtime
pub enum RuntimeError {
    //todo
}

impl Interpreter {
    // new interpreter constructor
    fn new() -> Interpreter {
        todo!()
    }
    
    // append new operation to the buffer 
    fn push_op(&mut self, op: &Operation) {
        todo!()
    }
    // append operations to the buffer
    fn push_ops(&mut self, ops: &Vec<Operation>) {
        todo!()
    }
    
    // execute and pop the first operation in the buffer
    fn execute_next(&mut self) -> Option<RuntimeError> {
        todo!()
    }
    // execute all operations and clear the buffer
    fn execute_all(&mut self) -> Option<RuntimeError> {
        todo!()
    }

    // pop last n (or less if not possible) pushed operations
    fn pop_buffer(&mut self, n: usize) {
        todo!()
    }
    
    // return number of buffered operations
    fn buffer_size(&self) -> usize {
        todo!()
    }
}