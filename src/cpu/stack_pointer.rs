const STACK_ADDR: u16 = 0x0100;
const STACK_POINTER_RESET: u8 = 0xfd;

/// One-byte stack pointer.
#[derive(Debug, Clone, Copy)]
pub struct StackPointer(u8);

impl Default for StackPointer {
    /// Default the stack pointer to the top of the stack address space in
    /// memory.
    fn default() -> Self {
        Self(STACK_POINTER_RESET)
    }
}

impl From<StackPointer> for u8 {
    fn from(s: StackPointer) -> Self {
        s.0
    }
}

impl From<StackPointer> for u16 {
    fn from(s: StackPointer) -> Self {
        STACK_ADDR + u16::from(s.0)
    }
}

impl From<u8> for StackPointer {
    fn from(i: u8) -> Self {
        Self(i)
    }
}

impl StackPointer {
    pub fn wrapping_add(self, rhs: u8) -> Self {
        Self(self.0.wrapping_add(rhs))
    }

    pub fn wrapping_sub(self, rhs: u8) -> Self {
        Self(self.0.wrapping_sub(rhs))
    }
}
