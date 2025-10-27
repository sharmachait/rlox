use crate::value::Types;

pub enum OpCode {
    OpReturn,
    // use when the value needs to be produced
    // has a single bytecode operand to determine which constant to load
    OpConstant,
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
            _ => OpCode::Unimplemented
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constant_pool: Vec<Types>
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
    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }
    pub fn add_constant(&mut self, value: Types) -> u8 {
        self.constant_pool.push(value);
        (self.constant_pool.len() - 1) as u8
    }
    pub fn free(&mut self) {
        self.code = Vec::new();
        self.constant_pool = Vec::new();
    }
}