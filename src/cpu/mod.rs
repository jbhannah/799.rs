use self::{
    addressing::AddressingMode,
    instructions::{Instruction, Instructions},
    memory::{Memory, MemoryValue, PROGRAM_COUNTER},
    status::Status,
};

mod addressing;
mod instructions;
mod memory;
mod opcodes;
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
        let ref opcodes = *opcodes::OPCODES_MAP;

        loop {
            let code: u8 = self.read_program_counter();

            let opcode = opcodes
                .get(&code)
                .expect(&format!("Opcode {:x} is not recognized", code));
            let operand = self.get_operand_address(&opcode.mode);

            match opcode.instruction {
                Instruction::ADC => todo!(),
                Instruction::AND => todo!(),
                Instruction::ASL => todo!(),
                Instruction::BCC => todo!(),
                Instruction::BCS => todo!(),
                Instruction::BEQ => todo!(),
                Instruction::BIT => todo!(),
                Instruction::BMI => todo!(),
                Instruction::BNE => todo!(),
                Instruction::BPL => todo!(),
                Instruction::BRK => return,
                Instruction::BVC => todo!(),
                Instruction::BVS => todo!(),
                Instruction::CLC => todo!(),
                Instruction::CLD => todo!(),
                Instruction::CLI => todo!(),
                Instruction::CLV => todo!(),
                Instruction::CMP => todo!(),
                Instruction::CPX => todo!(),
                Instruction::CPY => todo!(),
                Instruction::DEC => todo!(),
                Instruction::DEX => todo!(),
                Instruction::DEY => todo!(),
                Instruction::EOR => todo!(),
                Instruction::INC => todo!(),
                Instruction::INX => self.inx(),
                Instruction::INY => todo!(),
                Instruction::JMP => todo!(),
                Instruction::JSR => todo!(),
                Instruction::LDA => self.lda(operand.expect("Required operand is missing")),
                Instruction::LDX => todo!(),
                Instruction::LDY => todo!(),
                Instruction::LSR => todo!(),
                Instruction::NOP => todo!(),
                Instruction::ORA => todo!(),
                Instruction::PHA => todo!(),
                Instruction::PHP => todo!(),
                Instruction::PLA => todo!(),
                Instruction::PLP => todo!(),
                Instruction::ROL => todo!(),
                Instruction::ROR => todo!(),
                Instruction::RTI => todo!(),
                Instruction::RTS => todo!(),
                Instruction::SBC => todo!(),
                Instruction::SEC => todo!(),
                Instruction::SED => todo!(),
                Instruction::SEI => todo!(),
                Instruction::STA => todo!(),
                Instruction::STX => todo!(),
                Instruction::STY => todo!(),
                Instruction::TAX => self.tax(),
                Instruction::TAY => todo!(),
                Instruction::TSX => todo!(),
                Instruction::TXA => todo!(),
                Instruction::TXS => todo!(),
                Instruction::TYA => todo!(),
            }
        }
    }

    fn get_operand_address(&mut self, mode: &AddressingMode) -> Option<u16> {
        match mode {
            /* Return the program counter, and increment it manually since we'll
             * be reading it directly.
             */
            AddressingMode::Immediate => {
                let pc = Some(self.program_counter);
                self.program_counter += 1;
                pc
            }

            AddressingMode::ZeroPage => Some(self.read_program_counter::<u8>() as u16),
            AddressingMode::ZeroPageX => {
                Some(self.read_program_counter::<u8>().wrapping_add(self.index_x) as u16)
            }
            AddressingMode::ZeroPageY => {
                Some(self.read_program_counter::<u8>().wrapping_add(self.index_y) as u16)
            }

            AddressingMode::Absolute => Some(self.read_program_counter::<u16>()),
            AddressingMode::AbsoluteX => Some(
                self.read_program_counter::<u16>()
                    .wrapping_add(self.index_x as u16),
            ),
            AddressingMode::AbsoluteY => Some(
                self.read_program_counter::<u16>()
                    .wrapping_add(self.index_y as u16),
            ),

            AddressingMode::IndirectX => {
                let ptr = self.read_program_counter::<u8>().wrapping_add(self.index_x);
                Some(self.read_indirect(ptr))
            }
            AddressingMode::IndirectY => {
                let ptr: u8 = self.read_program_counter();
                Some(self.read_indirect(ptr).wrapping_add(self.index_y as u16))
            }

            AddressingMode::NoneAddressing => None,
        }
    }

    fn read_indirect(&self, ptr: u8) -> u16 {
        let lo: u8 = self.memory.read(ptr as u16);
        let hi: u8 = self.memory.read(ptr.wrapping_add(1) as u16);
        (hi as u16) << 8 | (lo as u16)
    }

    /**
     * Read the value at the address of the program counter, and increment the
     * counter by the number of bytes in the returned value.
     */
    fn read_program_counter<T: MemoryValue>(&mut self) -> T {
        let val: T = self.memory.read(self.program_counter);
        self.program_counter += T::BITS / 8;
        val
    }
}

impl Instructions for CPU {
    fn inx(&mut self) {
        self.index_x = self.index_x.wrapping_add(1);

        self.status.set_negative(self.index_x);
        self.status.set_zero(self.index_x);
    }

    fn lda(&mut self, operand: u16) {
        self.accumulator = self.memory.read(operand);

        self.status.set_negative(self.accumulator);
        self.status.set_zero(self.accumulator);
    }

    fn tax(&mut self) {
        self.index_x = self.accumulator;

        self.status.set_negative(self.index_x);
        self.status.set_zero(self.index_x);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa5_lda_zero_page() {
        let mut cpu = CPU::new();
        cpu.memory.write(0x10, 0x55 as u8);
        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.accumulator, 0x55);
    }

    #[test]
    fn test_0xa9_lda_immediate() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);

        assert_eq!(cpu.accumulator, 0x05);
        assert!(!cpu.status.contains(Status::Negative));
        assert!(!cpu.status.contains(Status::Zero));
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x80, 0x00]);

        assert_eq!(cpu.accumulator, 0x80);
        assert!(cpu.status.contains(Status::Negative));
        assert!(!cpu.status.contains(Status::Zero));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);

        assert_eq!(cpu.accumulator, 0x00);
        assert!(!cpu.status.contains(Status::Negative));
        assert!(cpu.status.contains(Status::Zero))
    }

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.index_x, 0x0a)
    }

    #[test]
    fn test_0xe8_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.index_x, 1);
    }
}
