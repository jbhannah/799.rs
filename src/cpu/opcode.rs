use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use num_derive::FromPrimitive;

#[derive(Debug, PartialEq)]
pub enum OpCodeError {
    InvalidCode(u8),
}

impl Error for OpCodeError {}

impl Display for OpCodeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InvalidCode(code) => write!(f, "Invalid op code \"{:x}\"", code),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, Hash, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    AdcImmediate = 0x69,
    AdcZeroPage = 0x65,
    AdcZeroPageX = 0x75,
    AdcAbsolute = 0x6d,
    AdcAbsoluteX = 0x7d,
    AdcAbsoluteY = 0x79,
    AdcIndirectX = 0x61,
    AdcIndirectY = 0x71,

    AndImmediate = 0x29,
    AndZeroPage = 0x25,
    AndZeroPageX = 0x35,
    AndAbsolute = 0x2d,
    AndAbsoluteX = 0x3d,
    AndAbsoluteY = 0x39,
    AndIndirectX = 0x21,
    AndIndirectY = 0x31,

    AslNoneAddressing = 0x0a,
    AslZeroPage = 0x06,
    AslZeroPageX = 0x16,
    AslAbsolute = 0x0e,
    AslAbsoluteX = 0x1e,

    BccRelative = 0x90,
    BcsRelative = 0xb0,
    BeqRelative = 0xf0,
    BmiRelative = 0x30,
    BneRelative = 0xd0,
    BplRelative = 0x10,
    BvcRelative = 0x50,
    BvsRelative = 0x70,

    BitZeroPage = 0x24,
    BitAbsolute = 0x2c,

    BrkNoneAddressing = 0x00,

    ClcNoneAddressing = 0x18,
    CldNoneAddressing = 0xd8,
    CliNoneAddressing = 0x58,
    ClvNoneAddressing = 0xb8,

    CmpImmediate = 0xc9,
    CmpZeroPage = 0xc5,
    CmpZeroPageX = 0xd5,
    CmpAbsolute = 0xcd,
    CmpAbsoluteX = 0xdd,
    CmpAbsoluteY = 0xd9,
    CmpIndirectX = 0xc1,
    CmpIndirectY = 0xd1,

    CpxImmediate = 0xe0,
    CpxZeroPage = 0xe4,
    CpxAbsolute = 0xec,

    CpyImmediate = 0xc0,
    CpyZeroPage = 0xc4,
    CpyAbsolute = 0xcc,

    DecZeroPage = 0xc6,
    DecZeroPageX = 0xd6,
    DecAbsolute = 0xce,
    DecAbsoluteX = 0xde,

    DexNoneAddressing = 0xca,
    DeyNoneAddressing = 0x88,

    EorImmediate = 0x49,
    EorZeroPage = 0x45,
    EorZeroPageX = 0x55,
    EorAbsolute = 0x4d,
    EorAbsoluteX = 0x5d,
    EorAbsoluteY = 0x59,
    EorIndirectX = 0x41,
    EorIndirectY = 0x51,

    IncZeroPage = 0xe6,
    IncZeroPageX = 0xf6,
    IncAbsolute = 0xee,
    IncAbsoluteX = 0xfe,

    InxNoneAddressing = 0xe8,
    InyNoneAddressing = 0xc8,

    JmpAbsolute = 0x4c,
    JmpIndirect = 0x6c,

    JsrAbsolute = 0x20,

    LdaImmediate = 0xa9,
    LdaZeroPage = 0xa5,
    LdaZeroPageX = 0xb5,
    LdaAbsolute = 0xad,
    LdaAbsoluteX = 0xbd,
    LdaAbsoluteY = 0xb9,
    LdaIndirectX = 0xa1,
    LdaIndirectY = 0xb1,

    LdxImmediate = 0xa2,
    LdxZeroPage = 0xa6,
    LdxZeroPageY = 0xb6,
    LdxAbsolute = 0xae,
    LdxAbsoluteY = 0xbe,

    LdyImmediate = 0xa0,
    LdyZeroPage = 0xa4,
    LdyZeroPageY = 0xb4,
    LdyAbsolute = 0xac,
    LdyAbsoluteY = 0xbc,

    LsrNoneAddressing = 0x4a,
    LsrZeroPage = 0x46,
    LsrZeroPageX = 0x56,
    LsrAbsolute = 0x4e,
    LsrAbsoluteX = 0x5e,

    NopNoneAddressing = 0xea,

    OraImmediate = 0x09,
    OraZeroPage = 0x05,
    OraZeroPageX = 0x15,
    OraAbsolute = 0x0d,
    OraAbsoluteX = 0x1d,
    OraAbsoluteY = 0x19,
    OraIndirectX = 0x01,
    OraIndirectY = 0x11,

    PhaNoneAddressing = 0x48,
    PhpNoneAddressing = 0x08,
    PlaNoneAddressing = 0x68,
    PlpNoneAddressing = 0x28,

    RolNoneAddressing = 0x2a,
    RolZeroPage = 0x26,
    RolZeroPageX = 0x36,
    RolAbsolute = 0x2e,
    RolAbsoluteX = 0x3e,

    RorNoneAddressing = 0x6a,
    RorZeroPage = 0x66,
    RorZeroPageX = 0x76,
    RorAbsolute = 0x6e,
    RorAbsoluteX = 0x7e,

    RtiNoneAddressing = 0x40,
    RtsNoneAddressing = 0x60,

    SbcImmediate = 0xe9,
    SbcZeroPage = 0xe5,
    SbcZeroPageX = 0xf5,
    SbcAbsolute = 0xed,
    SbcAbsoluteX = 0xfd,
    SbcAbsoluteY = 0xf9,
    SbcIndirectX = 0xe1,
    SbcIndirectY = 0xf1,

    SecNoneAddressing = 0x38,
    SedNoneAddressing = 0xf8,
    SeiNoneAddressing = 0x78,

    StaZeroPage = 0x85,
    StaZeroPageX = 0x95,
    StaAbsolute = 0x8d,
    StaAbsoluteX = 0x9d,
    StaAbsoluteY = 0x99,
    StaIndirectX = 0x81,
    StaIndirectY = 0x91,

    StxZeroPage = 0x86,
    StxZeroPageY = 0x96,
    StxAbsolute = 0x8e,

    StyZeroPage = 0x84,
    StyZeroPageX = 0x94,
    StyAbsolute = 0x8c,

    TaxNoneAddressing = 0xaa,
    TayNoneAddressing = 0xa8,
    TsxNoneAddressing = 0xba,
    TxaNoneAddressing = 0x8a,
    TxsNoneAddressing = 0x9a,
    TyaNoneAddressing = 0x98,
}

impl TryFrom<u8> for OpCode {
    type Error = OpCodeError;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        match num::FromPrimitive::from_u8(code) {
            Some(op) => Ok(op),
            None => Err(Self::Error::InvalidCode(code)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        value as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_u8_valid() {
        let code = 0x00_u8;

        assert_eq!(OpCode::try_from(code), Ok(OpCode::BrkNoneAddressing));
    }

    #[test]
    fn test_from_u8_invalid() {
        let code = 0xff_u8;

        assert_eq!(OpCode::try_from(code), Err(OpCodeError::InvalidCode(code)));
    }

    #[test]
    fn test_into_u8() {
        let op = OpCode::BrkNoneAddressing;

        assert_eq!(op as u8, 0x00);
    }
}
