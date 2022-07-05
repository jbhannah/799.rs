use std::collections::HashMap;

use super::instructions::Instruction;
use lazy_static::lazy_static;

#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    NoneAddressing,
}

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

        OpCode::new(0x29, Instruction::AND, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, Instruction::AND, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, Instruction::AND, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x2d, Instruction::AND, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3d, Instruction::AND, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x39, Instruction::AND, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x21, Instruction::AND, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x31, Instruction::AND, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x0a, Instruction::ASL, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x06, Instruction::ASL, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x16, Instruction::ASL, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x0e, Instruction::ASL, 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1e, Instruction::ASL, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x90, Instruction::BCC, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0xb0, Instruction::BCS, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0xf0, Instruction::BEQ, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x30, Instruction::BMI, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0xd0, Instruction::BNE, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x10, Instruction::BPL, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x50, Instruction::BVC, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x70, Instruction::BVS, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),

        OpCode::new(0x24, Instruction::BIT, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x2c, Instruction::BIT, 3, 4, AddressingMode::Absolute),

        OpCode::new(0x00, Instruction::BRK, 1, 7, AddressingMode::NoneAddressing),

        OpCode::new(0x18, Instruction::CLC, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xd8, Instruction::CLD, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x58, Instruction::CLI, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xb8, Instruction::CLV, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xc9, Instruction::CMP, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc5, Instruction::CMP, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xd5, Instruction::CMP, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xcd, Instruction::CMP, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xdd, Instruction::CMP, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xd9, Instruction::CMP, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xc1, Instruction::CMP, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xd1, Instruction::CMP, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xe0, Instruction::CPX, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe4, Instruction::CPX, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xec, Instruction::CPX, 3, 4, AddressingMode::Absolute),

        OpCode::new(0xc0, Instruction::CPY, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc4, Instruction::CPY, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xcc, Instruction::CPY, 3, 4, AddressingMode::Absolute),

        OpCode::new(0xc6, Instruction::DEC, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xd6, Instruction::DEC, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xce, Instruction::DEC, 3, 6, AddressingMode::Absolute),
        OpCode::new(0xde, Instruction::DEC, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xca, Instruction::DEX, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x88, Instruction::DEY, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x49, Instruction::EOR, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, Instruction::EOR, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, Instruction::EOR, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x4d, Instruction::EOR, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5d, Instruction::EOR, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x59, Instruction::EOR, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x41, Instruction::EOR, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x51, Instruction::EOR, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xe6, Instruction::INC, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xf6, Instruction::INC, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xee, Instruction::INC, 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfe, Instruction::INC, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xe8, Instruction::INX, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xc8, Instruction::INY, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x4c, Instruction::JMP, 3, 3, AddressingMode::Absolute),
        OpCode::new(0x6c, Instruction::JMP, 3, 5, AddressingMode::Indirect),

        OpCode::new(0x20, Instruction::JSR, 3, 6, AddressingMode::Absolute),

        OpCode::new(0xa9, Instruction::LDA, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, Instruction::LDA, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, Instruction::LDA, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xad, Instruction::LDA, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, Instruction::LDA, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xb9, Instruction::LDA, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xa1, Instruction::LDA, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xb1, Instruction::LDA, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xa2, Instruction::LDX, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa6, Instruction::LDX, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb6, Instruction::LDX, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0xae, Instruction::LDX, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbe, Instruction::LDX, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),

        OpCode::new(0xa0, Instruction::LDY, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa4, Instruction::LDY, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb4, Instruction::LDY, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0xac, Instruction::LDY, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbc, Instruction::LDY, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),

        OpCode::new(0x09, Instruction::ORA, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, Instruction::ORA, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, Instruction::ORA, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x0d, Instruction::ORA, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1d, Instruction::ORA, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x19, Instruction::ORA, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x01, Instruction::ORA, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x11, Instruction::ORA, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x2a, Instruction::ROL, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x26, Instruction::ROL, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x36, Instruction::ROL, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x2e, Instruction::ROL, 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3e, Instruction::ROL, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x6a, Instruction::ROR, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x66, Instruction::ROR, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x76, Instruction::ROR, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x6e, Instruction::ROR, 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7e, Instruction::ROR, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x60, Instruction::RTS, 1, 6, AddressingMode::NoneAddressing),

        OpCode::new(0xe9, Instruction::SBC, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, Instruction::SBC, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xf5, Instruction::SBC, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xed, Instruction::SBC, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, Instruction::SBC, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xf9, Instruction::SBC, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xe1, Instruction::SBC, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xf1, Instruction::SBC, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x38, Instruction::SEC, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xf8, Instruction::SED, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x78, Instruction::SEI, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x85, Instruction::STA, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, Instruction::STA, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8d, Instruction::STA, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, Instruction::STA, 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(0x99, Instruction::STA, 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(0x81, Instruction::STA, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x91, Instruction::STA, 2, 6, AddressingMode::IndirectY),

        OpCode::new(0x86, Instruction::STX, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x96, Instruction::STX, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0x8e, Instruction::STX, 3, 4, AddressingMode::Absolute),

        OpCode::new(0x84, Instruction::STY, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x94, Instruction::STY, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8c, Instruction::STY, 3, 4, AddressingMode::Absolute),

        OpCode::new(0xaa, Instruction::TAX, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xa8, Instruction::TAY, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xba, Instruction::TSX, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x8a, Instruction::TXA, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x9a, Instruction::TXS, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x98, Instruction::TYA, 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for op in &*CPU_OPS_CODES {
            map.insert(op.code, op);
        }
        map
    };
}
