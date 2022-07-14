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
        OpCode::new(0x69, Instruction::Adc, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, Instruction::Adc, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, Instruction::Adc, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x6d, Instruction::Adc, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, Instruction::Adc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x79, Instruction::Adc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x61, Instruction::Adc, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x71, Instruction::Adc, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x29, Instruction::And, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, Instruction::And, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, Instruction::And, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x2d, Instruction::And, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3d, Instruction::And, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x39, Instruction::And, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x21, Instruction::And, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x31, Instruction::And, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x0a, Instruction::Asl, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x06, Instruction::Asl, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x16, Instruction::Asl, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x0e, Instruction::Asl, 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1e, Instruction::Asl, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x90, Instruction::Bcc, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0xb0, Instruction::Bcs, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0xf0, Instruction::Beq, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x30, Instruction::Bmi, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0xd0, Instruction::Bne, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x10, Instruction::Bpl, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x50, Instruction::Bvc, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCode::new(0x70, Instruction::Bvs, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),

        OpCode::new(0x24, Instruction::Bit, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x2c, Instruction::Bit, 3, 4, AddressingMode::Absolute),

        OpCode::new(0x00, Instruction::Brk, 1, 7, AddressingMode::NoneAddressing),

        OpCode::new(0x18, Instruction::Clc, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xd8, Instruction::Cld, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x58, Instruction::Cli, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xb8, Instruction::Clv, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xc9, Instruction::Cmp, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc5, Instruction::Cmp, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xd5, Instruction::Cmp, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xcd, Instruction::Cmp, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xdd, Instruction::Cmp, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xd9, Instruction::Cmp, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xc1, Instruction::Cmp, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xd1, Instruction::Cmp, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xe0, Instruction::Cpx, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe4, Instruction::Cpx, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xec, Instruction::Cpx, 3, 4, AddressingMode::Absolute),

        OpCode::new(0xc0, Instruction::Cpy, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc4, Instruction::Cpy, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xcc, Instruction::Cpy, 3, 4, AddressingMode::Absolute),

        OpCode::new(0xc6, Instruction::Dec, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xd6, Instruction::Dec, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xce, Instruction::Dec, 3, 6, AddressingMode::Absolute),
        OpCode::new(0xde, Instruction::Dec, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xca, Instruction::Dex, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x88, Instruction::Dey, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x49, Instruction::Eor, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, Instruction::Eor, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, Instruction::Eor, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x4d, Instruction::Eor, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5d, Instruction::Eor, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x59, Instruction::Eor, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x41, Instruction::Eor, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x51, Instruction::Eor, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xe6, Instruction::Inc, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xf6, Instruction::Inc, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xee, Instruction::Inc, 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfe, Instruction::Inc, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xe8, Instruction::Inx, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xc8, Instruction::Iny, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x4c, Instruction::Jmp, 3, 3, AddressingMode::Absolute),
        OpCode::new(0x6c, Instruction::Jmp, 3, 5, AddressingMode::Indirect),

        OpCode::new(0x20, Instruction::Jsr, 3, 6, AddressingMode::Absolute),

        OpCode::new(0xa9, Instruction::Lda, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, Instruction::Lda, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, Instruction::Lda, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xad, Instruction::Lda, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, Instruction::Lda, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xb9, Instruction::Lda, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xa1, Instruction::Lda, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xb1, Instruction::Lda, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0xa2, Instruction::Ldx, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa6, Instruction::Ldx, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb6, Instruction::Ldx, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0xae, Instruction::Ldx, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbe, Instruction::Ldx, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),

        OpCode::new(0xa0, Instruction::Ldy, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa4, Instruction::Ldy, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb4, Instruction::Ldy, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0xac, Instruction::Ldy, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbc, Instruction::Ldy, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),

        OpCode::new(0xea, Instruction::Nop, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x09, Instruction::Ora, 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, Instruction::Ora, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, Instruction::Ora, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x0d, Instruction::Ora, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1d, Instruction::Ora, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0x19, Instruction::Ora, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0x01, Instruction::Ora, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x11, Instruction::Ora, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x2a, Instruction::Rol, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x26, Instruction::Rol, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x36, Instruction::Rol, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x2e, Instruction::Rol, 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3e, Instruction::Rol, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x6a, Instruction::Ror, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x66, Instruction::Ror, 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x76, Instruction::Ror, 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x6e, Instruction::Ror, 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7e, Instruction::Ror, 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x60, Instruction::Rts, 1, 6, AddressingMode::NoneAddressing),

        OpCode::new(0xe9, Instruction::Sbc, 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, Instruction::Sbc, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xf5, Instruction::Sbc, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xed, Instruction::Sbc, 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, Instruction::Sbc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCode::new(0xf9, Instruction::Sbc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCode::new(0xe1, Instruction::Sbc, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xf1, Instruction::Sbc, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCode::new(0x38, Instruction::Sec, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xf8, Instruction::Sed, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x78, Instruction::Sei, 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0x85, Instruction::Sta, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, Instruction::Sta, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8d, Instruction::Sta, 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, Instruction::Sta, 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(0x99, Instruction::Sta, 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(0x81, Instruction::Sta, 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x91, Instruction::Sta, 2, 6, AddressingMode::IndirectY),

        OpCode::new(0x86, Instruction::Stx, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x96, Instruction::Stx, 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0x8e, Instruction::Stx, 3, 4, AddressingMode::Absolute),

        OpCode::new(0x84, Instruction::Sty, 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x94, Instruction::Sty, 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8c, Instruction::Sty, 3, 4, AddressingMode::Absolute),

        OpCode::new(0xaa, Instruction::Tax, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xa8, Instruction::Tay, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xba, Instruction::Tsx, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x8a, Instruction::Txa, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x9a, Instruction::Txs, 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x98, Instruction::Tya, 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for op in &*CPU_OPS_CODES {
            map.insert(op.code, op);
        }
        map
    };
}
