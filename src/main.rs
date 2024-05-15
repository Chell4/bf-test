use std::io::{
    self,
    Write
};

use bf_test::{backend::{self}, frontend};

fn main() {
    let mut command_history = Vec::new();
    let mut interpreter = backend::Interpreter::new();

    loop {
        print!(">>>: ");

        if let Err(err) = io::stdout().flush() {
            println!("Cannot flush stdout: {err}");
            interpreter = backend::Interpreter::new();
            continue;
        }

        let mut source = String::new();
        command_history.push(source.clone());
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