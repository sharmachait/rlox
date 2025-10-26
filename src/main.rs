use rlox::chunk::{Chunk};
use rlox::chunk::OpCode::OpReturn;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpReturn as u8);
    chunk.free();
}
