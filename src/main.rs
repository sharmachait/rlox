use rlox::{
    chunk::OpCode::{OpReturn},
    value,
    chunk::{Chunk},
    vm::VM,
};
use rlox::chunk::OpCode::{OpNegate, OpAdd, OpDivide};

fn main() {

    let mut vm: VM = VM::new();
    let mut chunk = Chunk::new();

    chunk.write_constant(value::Types::Val(1.2), 123);
    chunk.write_constant(value::Types::Val(3.4), 123);

    chunk.write(OpAdd as u8, 123);

    chunk.write_constant(value::Types::Val(5.6), 123);

    chunk.write(OpDivide as u8, 123);

    chunk.write(OpNegate as u8, 123);

    chunk.write(OpReturn as u8, 123);

    vm.interpret(&mut chunk);
    vm.free();
    chunk.free();
}
