use crate::transpiler::value::Value;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpReturn,
    // use when the value needs to be produced
    // has a single bytecode operand to determine which constant to load
    OpConstant,
    OpConstantLong,
    OpNil,
    OpTrue,
    OpFalse,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    Unimplemented
}

impl From<u8> for OpCode {
    // this allows us to let enumVal: OpCode = 0.into();
    // turn numbers to enum values because its all loosey goosey in c
    // and that loosey goosey ness is used a lot in c projects

    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
            1 => OpCode::OpConstant,
            2 => OpCode::OpConstantLong,
            3 => OpCode::OpNil,
            4 => OpCode::OpTrue,
            5 => OpCode::OpFalse,
            6 => OpCode::OpNegate,
            7 => OpCode::OpAdd,
            8 => OpCode::OpSubtract,
            9 => OpCode::OpMultiply,
            10 => OpCode::OpDivide,
            _ => OpCode::Unimplemented
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Byte {
    pub byte: u8,
    pub line: usize
}

impl Byte{
    pub fn new(byte: u8, line: usize) -> Self {
        Self {
            byte,
            line
        }
    }
}

pub struct Chunk {
    pub code: Vec<Byte>,
    pub constant_pool: Vec<Value>
}


// operands for a bytecode instruction are not the operands for the operator
// but flags that modify how the instruction behaves
// operands follow the op code immediately afterwards in the byte stream


impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constant_pool: Vec::new(),
        }
    }
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(Byte::new(byte, line));
    }
    pub fn write_constant(&mut self, val: Value, line: usize) {
        let constant_pool_index = self.add_constant(val);

        if constant_pool_index < 256 {
            self.write(OpCode::OpConstant as u8, line);
        }else {
            self.write(OpCode::OpConstantLong as u8, line);
        }

        if constant_pool_index >=256 {
            self.write((constant_pool_index & 0xFF) as u8, line);
            self.write(((constant_pool_index >> 8) & 0xFF) as u8, line);
            self.write(((constant_pool_index >> 16) & 0xFF) as u8, line);
            // 3 bytes as the book tells us to use 24 bits
        }else{
            self.write(constant_pool_index as u8, line);
        }
    }
    fn add_constant(&mut self, value: Value) -> usize {
        self.constant_pool.push(value);
        self.constant_pool.len() - 1
    }
    pub fn free(&mut self) {
        self.code = Vec::new();
        self.constant_pool = Vec::new();
    }
}