use crate::chunk::{Chunk, OpCode};

pub fn disassemble<T: ToString>(chunk: &mut Chunk, name: T){
    println!("== {} ==", name.to_string());

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}
fn disassemble_instruction(chunk: &mut Chunk, offset: usize) -> usize {
    print!("{offset:04} "); // not necessary but useful when we get to loops and if statements
    // alot of jumping around in the code

    let instruction: OpCode = chunk.code[offset].into();
    match instruction {
        OpCode::OpReturn => {
            simple_instruction("OP_RETURN", offset)
        },
        OpCode::Unimplemented =>{
            println!("Unknown opcode {}", instruction as u8);
            offset+1
        }
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset+1
}