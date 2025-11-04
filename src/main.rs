use rlox::virtual_machine::{
    vm::VM
};

use std::{env, io::{Write}, io, fs};
use std::process;
use rlox::transpiler::value::{Obj, Value};
use rlox::virtual_machine::vm::RunResult;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: clox [path]");
        process::exit(64);
    }
}

fn repl() {
    loop {
        let mut vm: VM = VM::new();
        let mut line = String::new();
        print!("> ");
        io::stdout().flush().unwrap(); // flush prompt

        line.clear();

        if io::stdin().read_line(&mut line).is_err() {
            println!();
            break;
        }

        // EOF (Ctrl+D / Ctrl+Z) returns 0 bytes
        if line.is_empty() {
            println!();
            break;
        }
        line = line.trim().to_string();
        vm.run_source(&mut line);
    }
}

fn run_file(path: &str) {
    let mut vm: VM = VM::new();

    let file = fs::read_to_string(path);
    let mut content = match file {
        Ok(c) => {
            c
        }
        Err(e) => {
            eprintln!("Could not read file \"{}\": {}", path, e);
            process::exit(74);
        }
    };

    let run_result: RunResult = vm.run_source(&mut content);

    match run_result {
        RunResult::Ok => {}
        RunResult::CompileError => {
            process::exit(65);
        }
        RunResult::RuntimeError(_) => {
            process::exit(70);
        }
    }
}