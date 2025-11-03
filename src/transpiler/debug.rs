use crate::transpiler::chunk::{Chunk, OpCode};
use crate::transpiler::value::{print_value};

pub fn disassemble<T: ToString>(chunk: &mut Chunk, name: T){
    println!("== {} ==", name.to_string());

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}
pub fn disassemble_instruction(chunk: & Chunk, offset: usize) -> usize {
    print!("{offset:04} ");
    // not necessary but useful when we get to loops and if statements
    // a lot of jumping around in the code
    if offset > 0 && chunk.code[offset].line == chunk.code[offset-1].line {
        print!("   | ");
    } else{
        print!("{:4} ", chunk.code[offset].line);
    }
    let instruction: OpCode = chunk.code[offset].byte.into();
    match instruction {
        OpCode::OpConstant => {
            constant_instruction("OP_CONSTANT", chunk, offset)
        },
        OpCode::OpConstantLong => {
            constant_instruction_long("OP_CONSTANT_LONG", chunk, offset)
        },
        OpCode::OpReturn => {
            simple_instruction("OP_RETURN", offset)
        },
        OpCode::OpNegate => {
            simple_instruction("OP_NEGATE", offset)
        },
        OpCode::Unimplemented => {
            println!("Unknown opcode {}", instruction as u8);
            offset + 1
        },
        OpCode::OpAdd => {
            simple_instruction("OP_ADD", offset)
        },
        OpCode::OpSubtract => {
            simple_instruction("OP_SUBTRACT", offset)
        },
        OpCode::OpMultiply => {
            simple_instruction("OP_MULTIPLY", offset)
        },
        OpCode::OpDivide => {
            simple_instruction("OP_DIVIDE", offset)
        },
        OpCode::OpNil => {
            simple_instruction("OpNil", offset)
        },
        OpCode::OpTrue => {
            simple_instruction("OpTrue", offset)
        },
        OpCode::OpFalse => {
            simple_instruction("OpFalse", offset)
        },
    }
}
fn constant_instruction_long(name: &str, chunk: & Chunk, offset: usize) -> usize {
    let byte1 = chunk.code[offset + 1].byte as usize;
    let byte2 = chunk.code[offset + 2].byte as usize;
    let byte3 = chunk.code[offset + 3].byte as usize;

    let constant = byte1 | (byte2 << 8) | (byte3 << 16);
    print!("{:<16} {:>4} '", name, constant);
    let val = &chunk.constant_pool[constant];
    print_value(val);
    println!("'");

    offset + 4  // OpCode + 3 operand bytes
}
fn constant_instruction(name: &str, chunk: & Chunk, offset: usize) -> usize {
    let constant_index = chunk.code[offset+ 1 ].byte;
    print!("{:<16} {:>4} '", name, constant_index);
    let val = &chunk.constant_pool[constant_index as usize];
    print_value(val);
    println!("'");
    offset + 2
}
fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}