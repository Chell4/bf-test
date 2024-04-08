mod interpreter;

use crate::frontend::Operation;

pub use interpreter::{
    Interpreter,
    InterpreterError,
};

pub enum BackendError {
    InterpreterError(InterpreterError)
}

// just interpret given vector of operations
pub fn interpret(ops: Vec<Operation>) -> Option<BackendError> {
    todo!()
}