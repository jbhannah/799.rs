const NES_TAG: [u8; 4] = [0x4e, 0x45, 0x53, 0x1a];

#[derive(Debug, PartialEq)]
pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
    FOUR_SCREEN,
}

impl Default for Mirroring {
    fn default() -> Self {
        Self::HORIZONTAL
    }
}

#[derive(Debug, Default)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &Vec<u8>) -> Self {
        Self {
            prg_rom: raw.clone(),
            ..Default::default()
        }
    }
}
