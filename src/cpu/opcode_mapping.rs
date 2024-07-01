use std::collections::HashMap;

use crate::cpu::opcode::OpCode;

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
pub struct OpCodeMapping {
    pub code: OpCode,
    pub instruction: Instruction,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCodeMapping {
    fn new(
        code: OpCode,
        instruction: Instruction,
        len: u8,
        cycles: u8,
        mode: AddressingMode,
    ) -> Self {
        OpCodeMapping {
            code,
            instruction,
            len,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCodeMapping> = vec![
        OpCodeMapping::new(OpCode::AdcImmediate, Instruction::Adc, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::AdcZeroPage, Instruction::Adc, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::AdcZeroPageX, Instruction::Adc, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::AdcAbsolute, Instruction::Adc, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::AdcAbsoluteX, Instruction::Adc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::AdcAbsoluteY, Instruction::Adc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::AdcIndirectX, Instruction::Adc, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::AdcIndirectY, Instruction::Adc, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::AndImmediate, Instruction::And, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::AndZeroPage, Instruction::And, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::AndZeroPageX, Instruction::And, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::AndAbsolute, Instruction::And, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::AndAbsoluteX, Instruction::And, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::AndAbsoluteY, Instruction::And, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::AndIndirectX, Instruction::And, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::AndIndirectY, Instruction::And, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::AslNoneAddressing, Instruction::Asl, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::AslZeroPage, Instruction::Asl, 2, 5, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::AslZeroPageX, Instruction::Asl, 2, 6, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::AslAbsolute, Instruction::Asl, 3, 6, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::AslAbsoluteX, Instruction::Asl, 3, 7, AddressingMode::AbsoluteX),

        OpCodeMapping::new(OpCode::BccRelative, Instruction::Bcc, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCodeMapping::new(OpCode::BcsRelative, Instruction::Bcs, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCodeMapping::new(OpCode::BeqRelative, Instruction::Beq, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCodeMapping::new(OpCode::BmiRelative, Instruction::Bmi, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCodeMapping::new(OpCode::BneRelative, Instruction::Bne, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCodeMapping::new(OpCode::BplRelative, Instruction::Bpl, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCodeMapping::new(OpCode::BvcRelative, Instruction::Bvc, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),
        OpCodeMapping::new(OpCode::BvsRelative, Instruction::Bvs, 2, 2 /* +1 if branch succeeds, +2 if to a new page */, AddressingMode::Relative),

        OpCodeMapping::new(OpCode::BitZeroPage, Instruction::Bit, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::BitAbsolute, Instruction::Bit, 3, 4, AddressingMode::Absolute),

        OpCodeMapping::new(OpCode::BrkNoneAddressing, Instruction::Brk, 1, 7, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::ClcNoneAddressing, Instruction::Clc, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::CldNoneAddressing, Instruction::Cld, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::CliNoneAddressing, Instruction::Cli, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::ClvNoneAddressing, Instruction::Clv, 1, 2, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::CmpImmediate, Instruction::Cmp, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::CmpZeroPage, Instruction::Cmp, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::CmpZeroPageX, Instruction::Cmp, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::CmpAbsolute, Instruction::Cmp, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::CmpAbsoluteX, Instruction::Cmp, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::CmpAbsoluteY, Instruction::Cmp, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::CmpIndirectX, Instruction::Cmp, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::CmpIndirectY, Instruction::Cmp, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::CpxImmediate, Instruction::Cpx, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::CpxZeroPage, Instruction::Cpx, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::CpxAbsolute, Instruction::Cpx, 3, 4, AddressingMode::Absolute),

        OpCodeMapping::new(OpCode::CpyImmediate, Instruction::Cpy, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::CpyZeroPage, Instruction::Cpy, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::CpyAbsolute, Instruction::Cpy, 3, 4, AddressingMode::Absolute),

        OpCodeMapping::new(OpCode::DecZeroPage, Instruction::Dec, 2, 5, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::DecZeroPageX, Instruction::Dec, 2, 6, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::DecAbsolute, Instruction::Dec, 3, 6, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::DecAbsoluteX, Instruction::Dec, 3, 7, AddressingMode::AbsoluteX),

        OpCodeMapping::new(OpCode::DexNoneAddressing, Instruction::Dex, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::DeyNoneAddressing, Instruction::Dey, 1, 2, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::EorImmediate, Instruction::Eor, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::EorZeroPage, Instruction::Eor, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::EorZeroPageX, Instruction::Eor, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::EorAbsolute, Instruction::Eor, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::EorAbsoluteX, Instruction::Eor, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::EorAbsoluteY, Instruction::Eor, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::EorIndirectX, Instruction::Eor, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::EorIndirectY, Instruction::Eor, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::IncZeroPage, Instruction::Inc, 2, 5, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::IncZeroPageX, Instruction::Inc, 2, 6, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::IncAbsolute, Instruction::Inc, 3, 6, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::IncAbsoluteX, Instruction::Inc, 3, 7, AddressingMode::AbsoluteX),

        OpCodeMapping::new(OpCode::InxNoneAddressing, Instruction::Inx, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::InyNoneAddressing, Instruction::Iny, 1, 2, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::JmpAbsolute, Instruction::Jmp, 3, 3, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::JmpIndirect, Instruction::Jmp, 3, 5, AddressingMode::Indirect),

        OpCodeMapping::new(OpCode::JsrAbsolute, Instruction::Jsr, 3, 6, AddressingMode::Absolute),

        OpCodeMapping::new(OpCode::LdaImmediate, Instruction::Lda, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::LdaZeroPage, Instruction::Lda, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::LdaZeroPageX, Instruction::Lda, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::LdaAbsolute, Instruction::Lda, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::LdaAbsoluteX, Instruction::Lda, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::LdaAbsoluteY, Instruction::Lda, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::LdaIndirectX, Instruction::Lda, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::LdaIndirectY, Instruction::Lda, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::LdxImmediate, Instruction::Ldx, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::LdxZeroPage, Instruction::Ldx, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::LdxZeroPageY, Instruction::Ldx, 2, 4, AddressingMode::ZeroPageY),
        OpCodeMapping::new(OpCode::LdxAbsolute, Instruction::Ldx, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::LdxAbsoluteY, Instruction::Ldx, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),

        OpCodeMapping::new(OpCode::LdyImmediate, Instruction::Ldy, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::LdyZeroPage, Instruction::Ldy, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::LdyZeroPageY, Instruction::Ldy, 2, 4, AddressingMode::ZeroPageY),
        OpCodeMapping::new(OpCode::LdyAbsolute, Instruction::Ldy, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::LdyAbsoluteY, Instruction::Ldy, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),

        OpCodeMapping::new(OpCode::LsrNoneAddressing, Instruction::Lsr, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::LsrZeroPage, Instruction::Lsr, 2, 5, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::LsrZeroPageX, Instruction::Lsr, 2, 6, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::LsrAbsolute, Instruction::Lsr, 3, 6, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::LsrAbsoluteX, Instruction::Lsr, 3, 7, AddressingMode::AbsoluteX),

        OpCodeMapping::new(OpCode::NopNoneAddressing, Instruction::Nop, 1, 2, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::OraImmediate, Instruction::Ora, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::OraZeroPage, Instruction::Ora, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::OraZeroPageX, Instruction::Ora, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::OraAbsolute, Instruction::Ora, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::OraAbsoluteX, Instruction::Ora, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::OraAbsoluteY, Instruction::Ora, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::OraIndirectX, Instruction::Ora, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::OraIndirectY, Instruction::Ora, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::PhaNoneAddressing, Instruction::Pha, 1, 3, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::PhpNoneAddressing, Instruction::Php, 1, 3, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::PlaNoneAddressing, Instruction::Pla, 1, 4, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::PlpNoneAddressing, Instruction::Plp, 1, 4, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::RolNoneAddressing, Instruction::Rol, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::RolZeroPage, Instruction::Rol, 2, 5, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::RolZeroPageX, Instruction::Rol, 2, 6, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::RolAbsolute, Instruction::Rol, 3, 6, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::RolAbsoluteX, Instruction::Rol, 3, 7, AddressingMode::AbsoluteX),

        OpCodeMapping::new(OpCode::RorNoneAddressing, Instruction::Ror, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::RorZeroPage, Instruction::Ror, 2, 5, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::RorZeroPageX, Instruction::Ror, 2, 6, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::RorAbsolute, Instruction::Ror, 3, 6, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::RorAbsoluteX, Instruction::Ror, 3, 7, AddressingMode::AbsoluteX),

        OpCodeMapping::new(OpCode::RtiNoneAddressing, Instruction::Rti, 1, 6, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::RtsNoneAddressing, Instruction::Rts, 1, 6, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::SbcImmediate, Instruction::Sbc, 2, 2, AddressingMode::Immediate),
        OpCodeMapping::new(OpCode::SbcZeroPage, Instruction::Sbc, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::SbcZeroPageX, Instruction::Sbc, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::SbcAbsolute, Instruction::Sbc, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::SbcAbsoluteX, Instruction::Sbc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::SbcAbsoluteY, Instruction::Sbc, 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::SbcIndirectX, Instruction::Sbc, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::SbcIndirectY, Instruction::Sbc, 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::SecNoneAddressing, Instruction::Sec, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::SedNoneAddressing, Instruction::Sed, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::SeiNoneAddressing, Instruction::Sei, 1, 2, AddressingMode::NoneAddressing),

        OpCodeMapping::new(OpCode::StaZeroPage, Instruction::Sta, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::StaZeroPageX, Instruction::Sta, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::StaAbsolute, Instruction::Sta, 3, 4, AddressingMode::Absolute),
        OpCodeMapping::new(OpCode::StaAbsoluteX, Instruction::Sta, 3, 5, AddressingMode::AbsoluteX),
        OpCodeMapping::new(OpCode::StaAbsoluteY, Instruction::Sta, 3, 5, AddressingMode::AbsoluteY),
        OpCodeMapping::new(OpCode::StaIndirectX, Instruction::Sta, 2, 6, AddressingMode::IndirectX),
        OpCodeMapping::new(OpCode::StaIndirectY, Instruction::Sta, 2, 6, AddressingMode::IndirectY),

        OpCodeMapping::new(OpCode::StxZeroPage, Instruction::Stx, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::StxZeroPageY, Instruction::Stx, 2, 4, AddressingMode::ZeroPageY),
        OpCodeMapping::new(OpCode::StxAbsolute, Instruction::Stx, 3, 4, AddressingMode::Absolute),

        OpCodeMapping::new(OpCode::StyZeroPage, Instruction::Sty, 2, 3, AddressingMode::ZeroPage),
        OpCodeMapping::new(OpCode::StyZeroPageX, Instruction::Sty, 2, 4, AddressingMode::ZeroPageX),
        OpCodeMapping::new(OpCode::StyAbsolute, Instruction::Sty, 3, 4, AddressingMode::Absolute),

        OpCodeMapping::new(OpCode::TaxNoneAddressing, Instruction::Tax, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::TayNoneAddressing, Instruction::Tay, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::TsxNoneAddressing, Instruction::Tsx, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::TxaNoneAddressing, Instruction::Txa, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::TxsNoneAddressing, Instruction::Txs, 1, 2, AddressingMode::NoneAddressing),
        OpCodeMapping::new(OpCode::TyaNoneAddressing, Instruction::Tya, 1, 2, AddressingMode::NoneAddressing),
    ];

    pub static ref OPCODES_MAP: HashMap<OpCode, &'static OpCodeMapping> = {
        let mut map = HashMap::new();
        for op in &*CPU_OPS_CODES {
            map.insert(op.code, op);
        }
        map
    };
}
