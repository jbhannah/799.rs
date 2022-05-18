pub const PROGRAM_ROM: usize = 0x8000;
pub const PROGRAM_COUNTER: u16 = 0xFFFC;
pub const MEMORY_SIZE: usize = 0xFFFF;

pub trait MemoryValue {
    const BITS: u16;
    fn read_from_memory(memory: &Memory, addr: usize) -> Self;
    fn write_to_memory(memory: &mut Memory, addr: usize, val: Self);
}

impl MemoryValue for u8 {
    const BITS: u16 = u8::BITS as u16;

    fn read_from_memory(memory: &Memory, addr: usize) -> Self {
        memory.0[addr]
    }

    fn write_to_memory(memory: &mut Memory, addr: usize, val: Self) {
        memory.0[addr] = val;
    }
}

impl MemoryValue for u16 {
    const BITS: u16 = u16::BITS as u16;

    fn read_from_memory(memory: &Memory, addr: usize) -> Self {
        u16::from_le_bytes(
            memory.0[addr..addr + 2]
                .try_into()
                .expect("invalid memory range"),
        )
    }

    fn write_to_memory(memory: &mut Memory, addr: usize, val: Self) {
        let (_, right) = memory.0.split_at_mut(addr);
        let (mid, _) = right.split_at_mut(2);

        mid.copy_from_slice(&val.to_le_bytes());
    }
}

#[derive(Debug)]
pub struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Memory([0; MEMORY_SIZE])
    }
}

impl Memory {
    pub fn load(&mut self, program: Vec<u8>) {
        self.0[PROGRAM_ROM..(PROGRAM_ROM + program.len())].copy_from_slice(&program[..]);
        self.write(PROGRAM_COUNTER, PROGRAM_ROM as u16);
    }

    pub fn read<T: MemoryValue>(&self, addr: u16) -> T {
        T::read_from_memory(self, addr as usize)
    }

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
        memory.write(0xBEEF, 0x42 as u8);

        assert_eq!(memory.0[0xBEEF], 0x42);
    }

    #[test]
    fn test_write_u16() {
        let mut memory = Memory::default();
        let addr = 0xBEEF;

        memory.write(addr, 0x4342 as u16);

        assert_eq!(memory.0[addr as usize], 0x42);
        assert_eq!(memory.0[addr as usize + 1], 0x43);
    }
}
