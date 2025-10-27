use rlox::chunk::{Chunk};
use rlox::chunk::OpCode::{OpConstant, OpReturn};
use rlox::debug::disassemble;
use rlox::value;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(value::Types::Val(1.2), 123);
    chunk.write(OpReturn as u8, 123);
    disassemble(&mut chunk, "test chunk");
    chunk.free();
}
