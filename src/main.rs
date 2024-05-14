use std::io::{self};

use bf_test::{backend::{self}, frontend};

fn main() {
    let mut interpreter = backend::Interpreter::new();

    loop {
        print!(">>> ");

        let mut source = String::new();
        if let Err(err) = io::stdin().read_line(&mut source) {
            println!("Cannot read line: {err}",);
            interpreter = backend::Interpreter::new();
            continue;
        }
        
        match frontend::analyze(source) {
            Err(err) => {
                println!("Analysis failed: {err}");
                interpreter = backend::Interpreter::new();
                continue;
            }
            Ok(ops) => {
                interpreter.push_ops(ops)
            }
        }
        
        if let Some(err) = interpreter.execute_all() {
            println!("Execution failed: {err}");
            interpreter = backend::Interpreter::new();
            continue;
        }
    }
}