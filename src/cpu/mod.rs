use self::{
    addressing::AddressingMode,
    instructions::{Instruction, Instructions},
    memory::{Memory, MemoryValue, RESET},
    status::Status,
};

mod addressing;
mod instructions;
mod memory;
mod opcodes;
mod status;

#[derive(Debug)]
pub struct StackPointer(u8);

impl Default for StackPointer {
    fn default() -> Self {
        Self(0xFD)
    }
}

#[derive(Debug, Default)]
pub struct CPU {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub stack_pointer: StackPointer,
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
        *self = Self {
            program_counter: self.memory.read(RESET),
            memory: self.memory,
            ..Default::default()
        }
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
                Instruction::ADC => self.adc(operand.expect("Required operand is missing")),
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
                Instruction::CLC => self.clc(),
                Instruction::CLD => self.cld(),
                Instruction::CLI => self.cli(),
                Instruction::CLV => self.clv(),
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
                Instruction::SBC => self.sbc(operand.expect("Required operand is missing")),
                Instruction::SEC => self.sec(),
                Instruction::SED => self.sed(),
                Instruction::SEI => self.sei(),
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

    fn add_to_accumulator(&mut self, value: u8) {
        let sum = self.accumulator as u16
            + value as u16
            + (if self.status.contains(Status::Carry) {
                1
            } else {
                0
            }) as u16;

        self.status.set_carry(sum);

        let result = sum as u8;
        self.status
            .set_overflow((value ^ result) & (result ^ self.accumulator) & 0x80 != 0);

        self.set_accumulator(result);
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

    fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;

        self.status.set_negative(self.accumulator);
        self.status.set_zero(self.accumulator);
    }
}

impl Instructions for CPU {
    fn adc(&mut self, operand: u16) {
        self.add_to_accumulator(self.memory.read(operand));
    }

    fn clc(&mut self) {
        self.status.set(Status::Carry, false);
    }

    fn cld(&mut self) {
        self.status.set(Status::Decimal, false);
    }

    fn cli(&mut self) {
        self.status.set(Status::InterruptDisable, false);
    }

    fn clv(&mut self) {
        self.status.set(Status::Overflow, false);
    }

    fn inx(&mut self) {
        self.index_x = self.index_x.wrapping_add(1);

        self.status.set_negative(self.index_x);
        self.status.set_zero(self.index_x);
    }

    fn lda(&mut self, operand: u16) {
        self.set_accumulator(self.memory.read(operand));
    }

    fn sbc(&mut self, operand: u16) {
        self.add_to_accumulator(
            (self.memory.read::<u8>(operand) as i8)
                .wrapping_neg()
                .wrapping_sub(1) as u8,
        );
    }

    fn sec(&mut self) {
        self.status.set(Status::Carry, true);
    }

    fn sed(&mut self) {
        self.status.set(Status::Decimal, true);
    }

    fn sei(&mut self) {
        self.status.set(Status::InterruptDisable, true);
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

    #[test]
    fn test_0x18_clc() {
        let mut cpu = CPU::new();
        cpu.status = cpu.status | Status::Carry;
        cpu.load_and_run(vec![0x18, 0x00]);

        assert!(!cpu.status.contains(Status::Carry));
    }

    #[test]
    fn test_0xd8_cld() {
        let mut cpu = CPU::new();
        cpu.status = cpu.status | Status::Decimal;
        cpu.load_and_run(vec![0xd8, 0x00]);

        assert!(!cpu.status.contains(Status::Decimal));
    }

    #[test]
    fn test_0x58_cli() {
        let mut cpu = CPU::new();
        cpu.status = cpu.status | Status::InterruptDisable;
        cpu.load_and_run(vec![0x58, 0x00]);

        assert!(!cpu.status.contains(Status::InterruptDisable));
    }

    #[test]
    fn test_0xb8_clv() {
        let mut cpu = CPU::new();
        cpu.status = cpu.status | Status::Overflow;
        cpu.load_and_run(vec![0xb8, 0x00]);

        assert!(!cpu.status.contains(Status::Overflow));
    }

    #[test]
    fn test_0x38_sec() {
        let mut cpu = CPU::new();
        cpu.status = cpu.status & Status::Carry.not();
        cpu.load_and_run(vec![0x38, 0x00]);

        assert!(cpu.status.contains(Status::Carry));
    }

    #[test]
    fn test_0xf8_sed() {
        let mut cpu = CPU::new();
        cpu.status = cpu.status & Status::Decimal.not();
        cpu.load_and_run(vec![0xf8, 0x00]);

        assert!(cpu.status.contains(Status::Decimal));
    }

    #[test]
    fn test_0x78_sei() {
        let mut cpu = CPU::new();
        cpu.status = cpu.status & Status::InterruptDisable.not();
        cpu.load_and_run(vec![0x78, 0x00]);

        assert!(cpu.status.contains(Status::InterruptDisable));
    }
}
