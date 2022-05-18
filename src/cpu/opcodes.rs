use std::collections::HashMap;

use super::{addressing::AddressingMode, instructions::Instruction};
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct OpCode {
    pub code: u8,
    pub instruction: Instruction,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, instruction: Instruction, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            instruction,
            len,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![
        OpCode::new(0x69, Instruction::ADC, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, Instruction::ADC, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, Instruction::ADC, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x6d, Instruction::ADC, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, Instruction::ADC, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x79, Instruction::ADC, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x61, Instruction::ADC, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x71, Instruction::ADC, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x00, Instruction::BRK, 1, 7, AddressingMode::NoneAddressing),

        OpCode::new(0xe8, Instruction::INX, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xa9, Instruction::LDA, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, Instruction::LDA, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, Instruction::LDA, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xad, Instruction::LDA, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, Instruction::LDA, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xb9, Instruction::LDA, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xa1, Instruction::LDA, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xb1, Instruction::LDA, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xe9, Instruction::SBC, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, Instruction::SBC, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xf5, Instruction::SBC, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xed, Instruction::SBC, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, Instruction::SBC, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xf9, Instruction::SBC, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xe1, Instruction::SBC, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xf1, Instruction::SBC, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xaa, Instruction::TAX, 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for op in &*CPU_OPS_CODES {
            map.insert(op.code, op);
        }
        map
    };
}
