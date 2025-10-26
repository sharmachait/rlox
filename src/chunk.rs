pub enum OpCode {
    OpReturn,
}

impl From<u8> for OpCode {
    // this allows us to let enumVal: OpCode = 0.into();
    // turn numbers to enum values because its all loosey goosey in c
    // and that loosey goosey ness is used a lot in c projects

    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::OpReturn,
            _ => unimplemented!("Unsupported Operation")
        }
    }
}

pub struct Chunk {
    code: Vec<u8>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new()
        }
    }
    pub fn write(&mut self,byte: u8){
        self.code.push(byte);
    }
    pub fn free(&mut self){
        self.code = Vec::new();
    }
}