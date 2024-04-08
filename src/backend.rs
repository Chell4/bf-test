mod interpreter;

use crate::frontend::Operation;

pub use interpreter::{
    Interpreter,
    InterpreterError,
    interpret,
};