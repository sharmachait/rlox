use std::io::Read;
use std::sync::{Mutex, OnceLock};
use crate::chunk::{Byte, Chunk, OpCode};
use crate::debug;
use crate::value::{print_value, Types};

pub struct VM<'a> { // “The Chunk reference stored in this VM must live at least as long as 'a.”
    chunk: Option<&'a mut Chunk>,
    instruction_pointer: usize,
    stack: Vec<Types>
}

impl<'a> VM<'a> {
    pub fn new() -> Self{
        Self {
            chunk: Option::None,
            instruction_pointer: 0,
            stack: Vec::new()
        }
    }
    pub fn free(&mut self) {
        self.chunk = None;
        self.instruction_pointer = 0;
        self.stack.clear();
    }
    pub fn interpret(&mut self, chunk: &'a mut Chunk) -> RunResult {
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
            println!("");

            let chunk: &Chunk = self.chunk.as_ref().unwrap();
            debug::disassemble_instruction(chunk, self.instruction_pointer);
            let byte: Option<Byte> = self.read_byte();
            if let Some(instruction) = byte {
                let op_code: OpCode = instruction.byte.into();
                match op_code {
                    OpCode::OpReturn => {
                        if let Some(value) = self.stack.pop() {
                            print_value(&value);
                            println!();
                        } else {
                            println!("Stack is empty!");
                        }
                        return RunResult::Ok;
                    }
                    OpCode::OpConstant => {
                        let constant = {
                            let c = self.read_constant();
                            c.clone()
                        };
                        self.stack.push(constant);
                        continue;
                    }
                    OpCode::OpConstantLong => {
                        let constant = {
                            let c = self.read_constant_long();
                            c.clone()
                        };
                        self.stack.push(constant);
                        continue;
                    }
                    OpCode::Unimplemented => {
                        return RunResult::RuntimeError(instruction);
                    }
                }
            }
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
        let constant = &(self.chunk.as_ref().unwrap().constant_pool[constant_index]);
        constant
    }

    fn read_constant_long(&mut self) -> &Types {
        let byte1:usize = self.read_byte().unwrap().byte.into();
        let byte2:usize = self.read_byte().unwrap().byte.into();
        let byte3:usize = self.read_byte().unwrap().byte.into();
        let constant_index = byte1 | (byte2 << 8) | (byte3 << 16);
        let constant = &(self.chunk.as_ref().unwrap().constant_pool[constant_index]);
        constant
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