#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Mos6502,
    Nes2A03,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Nes2A03
    }
}

impl Mode {
    pub fn program_rom(&self) -> usize {
        match self {
            Self::Mos6502 => 0x0600,
            Self::Nes2A03 => 0x8000,
        }
    }
}
