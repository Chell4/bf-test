pub mod interpreter;

use crate::frontend::Operation;

pub use interpreter::Interpreter;

// just interpret given vector of operations
pub fn interpret(ops: Vec<Operation>) -> Option<interpreter::RuntimeError> {
    todo!()
}