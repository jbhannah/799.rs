use super::opcode::OpCode;

pub struct Program(pub Vec<OpCode>);

impl Into<Vec<u8>> for Program {
    fn into(self) -> Vec<u8> {
        self.0.into_iter().map(|op| op.into()).collect::<Vec<u8>>()
    }
}
