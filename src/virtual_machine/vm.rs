// use std::io::Read;
// use std::sync::{Mutex, OnceLock};
use crate::transpiler::chunk::{Byte, Chunk, OpCode};
use crate::transpiler::debug;
use crate::transpiler::value::{print_value, Types};
use crate::lexer::lexer::{Scanner};
use crate::lexer::token_type::TokenType;
use crate::transpiler::Parser::compile;

pub struct VM { // “The Chunk reference stored in this VM must live at least as long as 'a.”
    chunk: Option<Chunk>,
    instruction_pointer: usize,
    stack: Vec<Types>
}

impl VM {
    pub fn new() -> Self{
        Self {
            chunk: Option::None,
            instruction_pointer: 0,
            stack: Vec::new()
        }
    }
    pub fn run_source(mut self, source: &mut String) -> RunResult {
        let mut chunk = Chunk::new();
        if compile(source, &mut chunk) {
            chunk.free();
            return RunResult::CompileError
        }
        self.chunk = Some(chunk);
        self.instruction_pointer = 0;
        let run_result = self.run();

        run_result
    }
    pub fn free(mut self) {
        self.chunk.unwrap().free();
        self.chunk = None;
        self.instruction_pointer = 0;
        self.stack.clear();
    }
    pub fn run_chunk(&mut self, chunk: Chunk) -> RunResult {
        self.chunk = Some(chunk);
        self.instruction_pointer = 0;
        self.run()
    }
    fn run(&mut self) -> RunResult {
        loop{
            print!("          ");
            for slot in &self.stack {
                print!("[ ");
                print_value(slot);
                print!(" ]");
            }
            println!();

            let chunk: &Chunk = self.chunk.as_ref().unwrap();
            debug::disassemble_instruction(chunk, self.instruction_pointer);
            let byte: Option<Byte> = self.read_byte();

            if let Some(instruction) = byte {
                let op_code: OpCode = instruction.byte.into();
                match op_code {
                    OpCode::OpReturn => {
                        self.handle_return();
                        return RunResult::Ok;
                    }
                    OpCode::OpConstant => {
                        self.handle_constant();
                        continue;
                    }
                    OpCode::OpConstantLong => {
                        self.handle_constant_long();
                        continue;
                    },
                    OpCode::OpNegate => {
                        self.handle_negate();
                    },
                    OpCode::Unimplemented => {
                        return RunResult::RuntimeError(instruction);
                    },
                    OpCode::OpAdd => {
                        self.handle_binary_op(OpCode::OpAdd);
                    },
                    OpCode::OpSubtract => {
                        self.handle_binary_op(OpCode::OpSubtract);
                    },
                    OpCode::OpMultiply => {
                        self.handle_binary_op(OpCode::OpMultiply);
                    },
                    OpCode::OpDivide => {
                        self.handle_binary_op(OpCode::OpDivide);
                    },
                }
            }
        }
    }
    fn handle_constant_long(&mut self) {
        let constant = {
            let c = self.read_constant_long();
            c.clone()
        };
        self.stack.push(constant);
    }
    fn handle_constant(&mut self) {
        let constant = {
            let c = self.read_constant();
            c.clone()
        };
        self.stack.push(constant);
    }
    fn handle_return(&mut self) {
        if let Some(value) = self.stack.pop() {
            print_value(&value);
            println!();
        } else {
            println!("Stack is empty!");
        }
    }
    fn read_byte(&mut self) -> Option<Byte> {
        let chunk = self.chunk.as_ref().unwrap();
        let byte = *chunk.code.get(self.instruction_pointer)?;
        self.instruction_pointer += 1;
        Some(byte)
    }
    fn read_constant(&mut self) -> &Types {
        let constant_index:usize = self.read_byte().unwrap().byte.into();
        &(self.chunk.as_ref().unwrap().constant_pool[constant_index])
    }
    fn read_constant_long(&mut self) -> &Types {
        let byte1:usize = self.read_byte().unwrap().byte.into();
        let byte2:usize = self.read_byte().unwrap().byte.into();
        let byte3:usize = self.read_byte().unwrap().byte.into();
        let constant_index = byte1 | (byte2 << 8) | (byte3 << 16);
        &(self.chunk.as_ref().unwrap().constant_pool[constant_index])
    }
    fn handle_negate(&mut self) {
        if let Some(Types::Val(v)) = self.stack.pop() {
            let new_val = Types::Val(-v);
            self.stack.push(new_val);
        }
    }

    fn handle_binary_op(&mut self, code: OpCode) {
        let (b, a) = match (self.stack.pop(), self.stack.pop()) {
            (Some(Types::Val(b)), Some(Types::Val(a))) => (b, a),
            _ => return, // Graceful exit if stack underflow or wrong types
        };

        let result = match code {
            OpCode::OpAdd => a + b,
            OpCode::OpSubtract => a - b,
            OpCode::OpMultiply => a * b,
            OpCode::OpDivide => a / b,
            _ => return, // Ignore unsupported opcodes
        };

        self.stack.push(Types::Val(result));
    }


}
pub enum RunResult<> {
    Ok,
    CompileError,
    RuntimeError(Byte)
}



// pub static VM_INSTANCE: OnceLock<Mutex<VM>> = OnceLock::new();
//
// pub fn init_vm() {
//     VM_INSTANCE.set(
//         Mutex::new(VM::new())
//     ).ok().expect("VM already initialized");
// }
//
// pub fn free_vm() {
//     if let Some(vm) = VM_INSTANCE.get() {
//         vm.lock().unwrap().free();
//     }
// }