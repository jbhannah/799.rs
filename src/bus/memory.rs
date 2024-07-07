use super::memory_value::MemoryValue;

pub trait Memory {
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
