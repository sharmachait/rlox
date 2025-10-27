use crate::chunk::{Chunk, OpCode};
use crate::value::{print_value};

pub fn disassemble<T: ToString>(chunk: &mut Chunk, name: T){
    println!("== {} ==", name.to_string());

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}
fn disassemble_instruction(chunk: &mut Chunk, offset: usize) -> usize {
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
        OpCode::OpReturn => {
            simple_instruction("OP_RETURN", offset)
        },
        OpCode::Unimplemented => {
            println!("Unknown opcode {}", instruction as u8);
            offset + 1
        }

    }
}
fn constant_instruction(name: &str, chunk: &mut Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset+ 1 ].byte;
    print!("{:<16} {:>4} '", name, constant);
    let val = &chunk.constant_pool[constant as usize];
    print_value(val);
    println!("'");
    offset + 2
}
fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}