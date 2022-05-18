use self::{
    memory::{Memory, PROGRAM_COUNTER},
    status::Status,
};

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
}
