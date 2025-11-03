// use std::io::Read;
// use std::sync::{Mutex, OnceLock};
use crate::transpiler::chunk::{Byte, Chunk, OpCode};
use crate::transpiler::debug;
use crate::transpiler::value::{print_value, Value};
use crate::lexer::lexer::{Scanner};
use crate::lexer::token_type::TokenType;
use crate::transpiler::chunk::OpCode::{OpGreater, OpLess};
use crate::transpiler::parser::compile;

pub struct VM { // “The Chunk reference stored in this VM must live at least as long as 'a.”
    chunk: Option<Chunk>,
    instruction_pointer: usize,
    stack: Vec<Value>
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
        if !compile(source, &mut chunk) {
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
                        if ! self.handle_negate(instruction){
                            return RunResult::RuntimeError(instruction);
                        }
                    },
                    OpCode::Unimplemented => {
                        return RunResult::RuntimeError(instruction);
                    },
                    OpCode::OpAdd => {
                        if !self.handle_binary_op(OpCode::OpAdd, instruction){
                            return RunResult::RuntimeError(instruction);
                        }
                    },
                    OpCode::OpSubtract => {
                        if !self.handle_binary_op(OpCode::OpSubtract, instruction){
                            return RunResult::RuntimeError(instruction);
                        }
                    },
                    OpCode::OpMultiply => {
                        if !self.handle_binary_op(OpCode::OpMultiply, instruction){
                            return RunResult::RuntimeError(instruction);
                        }
                    },
                    OpCode::OpDivide => {
                        if !self.handle_binary_op(OpCode::OpDivide, instruction){
                            return RunResult::RuntimeError(instruction);
                        }
                    },
                    OpCode::OpNil => {
                        self.stack.push(Value::Nil)
                    }
                    OpCode::OpTrue => {
                        self.stack.push(Value::Bool(true))
                    }
                    OpCode::OpFalse => {
                        self.stack.push(Value::Bool(false))
                    }
                    OpCode::OpNot => {
                        let v = self.stack.pop();
                        self.stack.push(Value::Bool(self.is_falsey(v)))
                    }
                    OpCode::OpEqual => {
                        self.handle_equal()
                    }
                    OpCode::OpGreater => {
                        self.handle_binary_op(OpGreater, instruction);
                    }
                    OpCode::OpLess => {
                        self.handle_binary_op(OpLess, instruction);
                    }
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
    fn read_constant(&mut self) -> &Value {
        let constant_index:usize = self.read_byte().unwrap().byte.into();
        &(self.chunk.as_ref().unwrap().constant_pool[constant_index])
    }
    fn read_constant_long(&mut self) -> &Value {
        let byte1:usize = self.read_byte().unwrap().byte.into();
        let byte2:usize = self.read_byte().unwrap().byte.into();
        let byte3:usize = self.read_byte().unwrap().byte.into();
        let constant_index = byte1 | (byte2 << 8) | (byte3 << 16);
        &(self.chunk.as_ref().unwrap().constant_pool[constant_index])
    }
    fn handle_negate(&mut self, instruction: Byte) -> bool {
        let peek = self.peek_stack_distance(0);
        if !peek.is_num() {
            self.runtime_error("Operant must be a number", instruction);
            return false;
        }
        if let Some(Value::Num(v)) = self.stack.pop() {
            let new_val = Value::Num(-v);
            self.stack.push(new_val);
        }
        true
    }

    fn peek_stack_distance(&self, distance: usize) -> &Value{
        &(self.stack)[self.stack.len() -1 -distance]
    }

    fn handle_binary_op(&mut self, code: OpCode, instruction: Byte) -> bool {
        let (b, a) = match (self.stack.pop(), self.stack.pop()) {
            (Some(Value::Num(b)), Some(Value::Num(a))) => (b, a),
            (Some(Value::Str(b)), Some(Value::Str(a))) => {
                return self.concat(b,a);
            },
            (Some(Value::Str(b)), Some(Value::Num(a))) => {
                return self.concat_string_num(b,a);
            },
            (Some(Value::Num(b)), Some(Value::Str(a))) => {
                return self.concat_num_string(b,a);
            },
            _ => {
                self.runtime_error("Operands must be numbers.", instruction);
                return false;
            },
        };

        let result = match code {
            OpCode::OpAdd => a + b,
            OpCode::OpSubtract => a - b,
            OpCode::OpMultiply => a * b,
            OpCode::OpDivide => a / b,
            OpCode::OpGreater => {
            self.stack.push(Value::Bool(a>b));
                return true;
            }
            OpCode::OpLess => {
                self.stack.push(Value::Bool(a<b));
                return true;
            }
            _ => return false, // Ignore unsupported opcodes
        };

        self.stack.push(Value::Num(result));
        true
    }

    fn runtime_error(&mut self, message: &str, instruction: Byte) {
        eprintln!("{}", message);
        let line = instruction.line;
        eprintln!("{} in script", line);
        self.stack.clear();
    }

    fn concat(&self, b: String, a: String) -> bool {
        todo!()
    }
    fn concat_num_string(&self, b: f64, a: String) -> bool {
        todo!()
    }
    fn concat_string_num(&self, b: String, a: f64) -> bool {
        todo!()
    }

    fn is_falsey(&self, val: Option<Value>) -> bool {
        let val = &(val.unwrap());
        Value::is_nil(val) || (Value::is_bool(val) && !val.as_bool().unwrap())
    }

    fn handle_equal(&mut self) {
        let (b, a) = (self.stack.pop(), self.stack.pop());
        let res = self.values_equal(a.unwrap(),b.unwrap());
        self.stack.push(Value::Bool(res));
    }

    fn values_equal(&self, a: Value, b: Value) -> bool {
        let is_equal = Value::equal_by_type(&a,&b);
        if  !is_equal {
            return false;
        }

        match a {
            Value::Num(x) => {
                a.as_number().unwrap() == b.as_number().unwrap()
            }
            Value::Str(x) => {
                todo!();
            }
            Value::Bool(x) => {
                a.as_bool().unwrap() == b.as_bool().unwrap()
            }
            Value::Nil => {true}
        }
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