use super::mode::Mode;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1fff;
const RAM_MIRRORS_MASK: u16 = 0x07ff;

const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3fff;

#[derive(Debug, Clone, Copy)]
pub struct Bus {
    cpu_vram: [u8; 2048],
}

pub trait BitSize {
    const BITS: u16;
}

impl BitSize for u8 {
    const BITS: u16 = u8::BITS as u16;
}

impl BitSize for u16 {
    const BITS: u16 = u16::BITS as u16;
}

pub trait MemoryValue<T: BitSize> {
    fn read(&self, addr: u16) -> T;

    fn write(&mut self, addr: u16, value: T);
}

trait Memory {
    fn mem_read(&self, addr: u16) -> u8;

    fn mem_write(&mut self, addr: u16, value: u8);
}

impl<T: Memory> MemoryValue<u8> for T {
    fn read(&self, addr: u16) -> u8 {
        self.mem_read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.mem_write(addr, value);
    }
}

impl<T: Memory> MemoryValue<u16> for T {
    fn read(&self, addr: u16) -> u16 {
        u16::from_le_bytes([self.mem_read(addr), self.mem_read(addr + 1)])
    }

    fn write(&mut self, addr: u16, value: u16) {
        for (index, byte) in value.to_le_bytes().into_iter().enumerate() {
            self.mem_write(addr + index as u16, byte);
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            cpu_vram: [0; 2048],
        }
    }
}

impl Memory for Bus {
    fn mem_read(&self, addr: u16) -> u8 {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & RAM_MIRRORS_MASK;
                self.cpu_vram[mirror_down_addr as usize]
            }
            _ => todo!("read from address ${:X} ignored", addr),
        }
    }

    fn mem_write(&mut self, addr: u16, value: u8) {
        match addr {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_addr = addr & RAM_MIRRORS_MASK;
                self.cpu_vram[mirror_down_addr as usize] = value;
            }
            _ => todo!("write to address ${:X} ignored", addr),
        }
    }
}

impl Bus {
    pub fn load(&mut self, program: Vec<u8>, mode: Mode) {
        let program_rom: usize = mode.program_rom().into();

        self.cpu_vram[program_rom..(program_rom + program.len())].copy_from_slice(&program[..]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_u8() {
        let mut bus = Bus::default();
        bus.cpu_vram[0x0000] = 0x42;

        let value: u8 = bus.read(0x0000);

        assert_eq!(value, 0x42);
    }

    #[test]
    fn test_read_u8_mirror() {
        let mut bus = Bus::default();
        bus.cpu_vram[0x0000] = 0x42;

        let value: u8 = bus.read(0x0800);

        assert_eq!(value, 0x42);
    }

    #[test]
    fn test_read_u16() {
        let mut bus = Bus::default();
        let addr = 0x0000;

        bus.cpu_vram[addr] = 0xef;
        bus.cpu_vram[addr + 1] = 0xbe;

        let value: u16 = bus.read(addr as u16);

        assert_eq!(value, 0xbeef);
    }

    #[test]
    fn test_read_u16_mirror() {
        let mut bus = Bus::default();
        let addr = 0x0000;

        bus.cpu_vram[addr] = 0xef;
        bus.cpu_vram[addr + 1] = 0xbe;

        let value: u16 = bus.read(0x0800);

        assert_eq!(value, 0xbeef);
    }

    #[test]
    fn test_write_u8() {
        let mut bus = Bus::default();

        bus.write(0x0000, 0x42_u8);

        assert_eq!(bus.cpu_vram[0x0000], 0x42);
    }

    #[test]
    fn test_write_u8_mirror() {
        let mut bus = Bus::default();

        bus.write(0x0800, 0x42_u8);

        assert_eq!(bus.cpu_vram[0x0000], 0x42);
    }

    #[test]
    fn test_write_u16() {
        let mut bus = Bus::default();
        let addr: usize = 0x0000;

        bus.write(0x0000, 0xbeef_u16);

        assert_eq!(bus.cpu_vram[addr], 0xef);
        assert_eq!(bus.cpu_vram[addr + 1], 0xbe);
    }

    #[test]
    fn test_write_u16_mirror() {
        let mut bus = Bus::default();
        let addr: usize = 0x0000;

        bus.write(0x0800, 0xbeef_u16);

        assert_eq!(bus.cpu_vram[addr], 0xef);
        assert_eq!(bus.cpu_vram[addr + 1], 0xbe);
    }
}
