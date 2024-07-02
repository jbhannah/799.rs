#[derive(Debug, Clone, Copy)]
pub enum Mode {
    /// Unmodified MOS Technology 6502 processor
    Mos6502,
    /// Ricoh RP2A03, modified 6502 used in NTSC NES models
    Rp2A03,
    /// Ricoh RP2A07, modified 6502 used in PAL NES models
    Rp2A07,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Mos6502
    }
}

impl Mode {
    pub fn program_rom(&self) -> u16 {
        match self {
            Self::Mos6502 => 0x0600,
            Self::Rp2A03 | Self::Rp2A07 => 0x8000,
        }
    }
}
