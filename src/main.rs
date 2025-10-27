use rlox::chunk::{Chunk};
use rlox::chunk::OpCode::OpReturn;
use rlox::debug::disassemble;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpReturn as u8);
    disassemble(&mut chunk, "test chunk");
    chunk.free();
}
