pub const MEMORY_SIZE: usize = 0xFFFF;

#[derive(Debug)]
pub struct Memory([u8; MEMORY_SIZE]);

impl Default for Memory {
    fn default() -> Self {
        Memory([0; MEMORY_SIZE])
    }
}
