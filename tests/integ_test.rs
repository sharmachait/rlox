use rlox::transpiler::chunk::Chunk;
use rlox::transpiler::chunk::OpCode::{OpConstant, OpConstantLong, OpNegate, OpReturn, OpAdd, OpDivide};
use rlox::transpiler::debug::disassemble;
use rlox::transpiler::value;
use rlox::virtual_machine::vm::VM;

#[test]
fn disassemble_large_constant_pool(){
    let mut chunk = Chunk::new();

    for _i in 0..320 {
        chunk.write_constant(value::Value::Num(1.2), 123);
    }
    chunk.write(OpReturn as u8, 123);

    // Verify we have 320 constants
    assert_eq!(chunk.constant_pool.len(), 320);

    // First 256 constants should use OP_CONSTANT (2 bytes each)
    // Constant 0
    assert_eq!(chunk.code[0].byte, OpConstant as u8);
    assert_eq!(chunk.code[1].byte, 0);

    // Constant 255 (last one using OP_CONSTANT)
    let offset_255 = 255 * 2; // Each OP_CONSTANT is 2 bytes
    assert_eq!(chunk.code[offset_255].byte, OpConstant as u8);
    assert_eq!(chunk.code[offset_255 + 1].byte, 255);

    // Constant 256 (first OP_CONSTANT_LONG) should be at offset 512 (256 * 2)
    let offset_256 = 256 * 2;
    assert_eq!(chunk.code[offset_256].byte, OpConstantLong as u8);
    assert_eq!(chunk.code[offset_256 + 1].byte, 0);   // 256 & 0xFF = 0
    assert_eq!(chunk.code[offset_256 + 2].byte, 1);   // (256 >> 8) & 0xFF = 1
    assert_eq!(chunk.code[offset_256 + 3].byte, 0);   // (256 >> 16) & 0xFF = 0

    // Constant 257
    let offset_257 = 256 * 2 + 4; // After first OP_CONSTANT_LONG
    assert_eq!(chunk.code[offset_257].byte, OpConstantLong as u8);
    assert_eq!(chunk.code[offset_257 + 1].byte, 1);   // 257 & 0xFF = 1
    assert_eq!(chunk.code[offset_257 + 2].byte, 1);   // (257 >> 8) & 0xFF = 1
    assert_eq!(chunk.code[offset_257 + 3].byte, 0);   // (257 >> 16) & 0xFF = 0

    // Last instruction should be OP_RETURN at offset 768
    // 256 OP_CONSTANT (512 bytes) + 64 OP_CONSTANT_LONG (256 bytes) = 768
    assert_eq!(chunk.code[768].byte, OpReturn as u8);

    // Optional: print for visual verification (won't be captured but useful when running test)
    disassemble(&mut chunk, "test chunk");

    chunk.free();
}

#[test]
fn vm_negate_return(){
    let mut vm: VM = VM::new();

    let mut chunk = Chunk::new();
    chunk.write_constant(value::Value::Num(1.2), 123);
    chunk.write(OpNegate as u8, 123);

    chunk.write(OpReturn as u8, 123);

    vm.run_chunk(chunk);
    vm.free();
    
}

#[test]
fn vm_binary_expression_return(){
    let mut vm: VM = VM::new();
    let mut chunk = Chunk::new();

    chunk.write_constant(value::Value::Num(1.2), 123);
    chunk.write_constant(value::Value::Num(3.4), 123);

    chunk.write(OpAdd as u8, 123);

    chunk.write_constant(value::Value::Num(5.6), 123);

    chunk.write(OpDivide as u8, 123);

    chunk.write(OpNegate as u8, 123);

    chunk.write(OpReturn as u8, 123);

    vm.run_chunk(chunk);
    vm.free();
}