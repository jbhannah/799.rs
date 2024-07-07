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
