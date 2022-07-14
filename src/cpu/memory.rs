use super::mode::Mode;

pub const MEMORY_SIZE: usize = 0x10000;

pub const STACK: u16 = 0x00FD;
pub const RESET: u16 = 0xFFFC;
pub const INTERRUPT: u16 = 0xFFFE;

/// Trait to read and write values of different sizes into memory.
pub trait MemoryValue {
    /// The size of the implementing type, in bits.
    const BITS: u16;

    /// Read a value from the appropriate number of bytes in memory.
    fn read_from_memory(memory: &Memory, addr: usize) -> Self;

    /// Write a value to the appropriate number of bytes in memory.
    fn write_to_memory(memory: &mut Memory, addr: usize, val: Self);
}

impl MemoryValue for u8 {
    const BITS: u16 = u8::BITS as u16;

    /// Read a single byte from memory as a u8.
    fn read_from_memory(memory: &Memory, addr: usize) -> Self {
        memory.0[addr]
    }

    /// Write a u8 value to a single byte of memory.
    fn write_to_memory(memory: &mut Memory, addr: usize, val: Self) {
        memory.0[addr] = val;
    }
}

impl MemoryValue for u16 {
    const BITS: u16 = u16::BITS as u16;

    /// Read two bytes from memory as a little-endian u16.
    fn read_from_memory(memory: &Memory, addr: usize) -> Self {
        u16::from_le_bytes(
            memory.0[addr..addr + 2]
                .try_into()
                .expect("invalid memory range"),
        )
    }

    /// Write a u16 value in little-endian form to two bytes of memory.
    fn write_to_memory(memory: &mut Memory, addr: usize, val: Self) {
        let (_, right) = memory.0.split_at_mut(addr);
        let (mid, _) = right.split_at_mut(2);

        mid.copy_from_slice(&val.to_le_bytes());
    }
}

/// A 64 KiB memory register.
#[derive(Debug, Clone, Copy)]
pub struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Memory([0; MEMORY_SIZE])
    }
}

impl Memory {
    /// Load a program into the program ROM section of memory.
    pub fn load(&mut self, program: Vec<u8>, mode: Mode) {
        let program_rom = mode.program_rom();

        self.0[program_rom..(program_rom + program.len())].copy_from_slice(&program[..]);
        self.write(RESET, program_rom as u16);
    }

    /// Read a u8 or u16 from memory.
    pub fn read<T: MemoryValue>(&self, addr: u16) -> T {
        T::read_from_memory(self, addr as usize)
    }

    /// Write a u8 or u16 to memory.
    pub fn write<T: MemoryValue>(&mut self, addr: u16, val: T) {
        T::write_to_memory(self, addr as usize, val)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_u8() {
        let mut memory = Memory::default();
        memory.0[0xBEEF] = 0x42;

        assert_eq!(memory.read::<u8>(0xBEEF), 0x42);
    }

    #[test]
    fn test_read_u16() {
        let mut memory = Memory::default();
        let addr = 0xBEEF;

        memory.0[addr] = 0x42;
        memory.0[addr + 1] = 0x43;

        assert_eq!(memory.read::<u16>(addr as u16), 0x4342);
    }

    #[test]
    fn test_write_u8() {
        let mut memory = Memory::default();
        memory.write(0xBEEF, 0x42_u8);

        assert_eq!(memory.0[0xBEEF], 0x42);
    }

    #[test]
    fn test_write_u16() {
        let mut memory = Memory::default();
        let addr = 0xBEEF;

        memory.write(addr, 0x4342_u16);

        assert_eq!(memory.0[addr as usize], 0x42);
        assert_eq!(memory.0[addr as usize + 1], 0x43);
    }
}
