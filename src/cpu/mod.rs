use self::{
    addressing::AddressingMode,
    memory::{Memory, MemoryValue, PROGRAM_COUNTER},
    status::Status,
};

mod addressing;
mod memory;
mod status;

#[derive(Debug, Default)]
pub struct CPU {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub status: Status,
    memory: Memory,
}

impl CPU {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.load(program);
    }

    pub fn reset(&mut self) {
        self.accumulator = Default::default();
        self.index_x = Default::default();
        self.index_y = Default::default();
        self.stack_pointer = Default::default();
        self.status = Default::default();

        self.program_counter = self.memory.read(PROGRAM_COUNTER);
    }

    pub fn run(&mut self) {
        loop {
            let opcode: u8 = self.memory.read(self.program_counter);
            self.program_counter += 1;

            match opcode {
                0x00 => {
                    return;
                }
                _ => todo!(),
            }
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,

            AddressingMode::ZeroPage => self.read_program_counter::<u8>() as u16,
            AddressingMode::ZeroPageX => {
                self.read_program_counter::<u8>().wrapping_add(self.index_x) as u16
            }
            AddressingMode::ZeroPageY => {
                self.read_program_counter::<u8>().wrapping_add(self.index_y) as u16
            }

            AddressingMode::Absolute => self.read_program_counter::<u16>(),
            AddressingMode::AbsoluteX => self
                .read_program_counter::<u16>()
                .wrapping_add(self.index_x as u16),
            AddressingMode::AbsoluteY => self
                .read_program_counter::<u16>()
                .wrapping_add(self.index_y as u16),

            AddressingMode::IndirectX => {
                let ptr = self.read_program_counter::<u8>().wrapping_add(self.index_x);
                self.read_indirect(ptr)
            }
            AddressingMode::IndirectY => {
                let ptr: u8 = self.read_program_counter();
                self.read_indirect(ptr).wrapping_add(self.index_y as u16)
            }

            AddressingMode::NoneAddressing => panic!("mode {:?} is not supported", mode),
        }
    }

    fn read_indirect(&self, ptr: u8) -> u16 {
        let lo: u8 = self.memory.read(ptr as u16);
        let hi: u8 = self.memory.read(ptr.wrapping_add(1) as u16);
        (hi as u16) << 8 | (lo as u16)
    }

    fn read_program_counter<T: MemoryValue>(&self) -> T {
        self.memory.read(self.program_counter)
    }
}
