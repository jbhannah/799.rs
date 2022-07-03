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
