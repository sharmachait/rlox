use rlox::{
    chunk::OpCode::{OpConstant, OpReturn},
    debug::disassemble,
    value,
    chunk::{Chunk, OpCode},
    value::Types,
    vm::VM,
};

fn main() {

    let mut vm: VM = VM::new();

    let mut chunk = Chunk::new();
    chunk.write_constant(value::Types::Val(1.2), 123);
    // for _i in 0..320 {
    //     let val = 1.0 * _i as f64;
    //     chunk.write_constant(value::Types::Val(val), 123);
    // }
    chunk.write(OpReturn as u8, 123);
    // disassemble(&mut chunk, "test chunk");

    vm.interpret(&mut chunk);
    vm.free();
    chunk.free();
}
