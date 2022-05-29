use self::{
    instructions::{Instruction, Instructions},
    memory::{Memory, MemoryValue},
    opcodes::AddressingMode,
    status::Status,
};

mod instructions;
mod memory;
mod opcodes;
mod status;

#[derive(Debug, Clone, Copy)]
pub struct StackPointer(u8);

impl Default for StackPointer {
    fn default() -> Self {
        Self(memory::STACK as u8)
    }
}

impl Into<u8> for StackPointer {
    fn into(self) -> u8 {
        self.0
    }
}

impl Into<u16> for StackPointer {
    fn into(self) -> u16 {
        self.0 as u16
    }
}

impl StackPointer {
    pub fn advance(&mut self, offset: i16) {
        self.0 = (self.0 as i16 + offset) as u8;
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
            program_counter: self.memory.read(memory::RESET),
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

            let addr = self.get_operand_address(&opcode.mode);

            match opcode.instruction {
                Instruction::ADC => self.with_operand(Self::adc, addr),
                Instruction::AND => self.with_operand(Self::and, addr),
                Instruction::ASL => self.asl(addr),
                Instruction::BCC => self.with_operand(Self::bcc, addr),
                Instruction::BCS => self.with_operand(Self::bcs, addr),
                Instruction::BEQ => self.with_operand(Self::beq, addr),
                Instruction::BIT => self.with_operand(Self::bit, addr),
                Instruction::BMI => self.with_operand(Self::bmi, addr),
                Instruction::BNE => self.with_operand(Self::bne, addr),
                Instruction::BPL => self.with_operand(Self::bpl, addr),
                Instruction::BRK => self.brk(),
                Instruction::BVC => self.with_operand(Self::bvc, addr),
                Instruction::BVS => self.with_operand(Self::bvs, addr),
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
                Instruction::EOR => self.with_operand(Self::eor, addr),
                Instruction::INC => todo!(),
                Instruction::INX => self.inx(),
                Instruction::INY => todo!(),
                Instruction::JMP => todo!(),
                Instruction::JSR => todo!(),
                Instruction::LDA => self.with_operand(Self::lda, addr),
                Instruction::LDX => todo!(),
                Instruction::LDY => todo!(),
                Instruction::LSR => todo!(),
                Instruction::NOP => todo!(),
                Instruction::ORA => self.with_operand(Self::ora, addr),
                Instruction::PHA => todo!(),
                Instruction::PHP => todo!(),
                Instruction::PLA => todo!(),
                Instruction::PLP => todo!(),
                Instruction::ROL => todo!(),
                Instruction::ROR => todo!(),
                Instruction::RTI => todo!(),
                Instruction::RTS => todo!(),
                Instruction::SBC => self.with_operand(Self::sbc, addr),
                Instruction::SEC => self.sec(),
                Instruction::SED => self.sed(),
                Instruction::SEI => self.sei(),
                Instruction::STA => self.with_operand(Self::sta, addr),
                Instruction::STX => self.with_operand(Self::stx, addr),
                Instruction::STY => self.with_operand(Self::sty, addr),
                Instruction::TAX => self.tax(),
                Instruction::TAY => self.tay(),
                Instruction::TSX => self.tsx(),
                Instruction::TXA => self.txa(),
                Instruction::TXS => self.txs(),
                Instruction::TYA => self.tya(),
            }

            if self.program_counter == 0 {
                return;
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

    fn branch(&mut self, addr: u16, condition: bool) {
        if condition {
            self.program_counter = addr;
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

            AddressingMode::Relative => {
                let offset = self.read_program_counter::<u8>() as i8;
                Some(self.program_counter.wrapping_add(offset as u16))
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

    fn set_index_x(&mut self, value: u8) {
        self.index_x = value;

        self.status.set_negative(self.index_x);
        self.status.set_zero(self.index_x);
    }

    fn set_index_y(&mut self, value: u8) {
        self.index_y = value;

        self.status.set_negative(self.index_y);
        self.status.set_zero(self.index_y);
    }

    /*
    fn stack_pop<T: MemoryValue>(&mut self) -> T {
        let val: T = self.memory.read(self.stack_pointer.into());
        self.stack_pointer.advance(T::BITS as i16 / -8);
        val
    }
    */

    fn stack_push<T: MemoryValue>(&mut self, value: T) {
        self.memory.write(self.stack_pointer.into(), value);
        self.stack_pointer.advance(T::BITS as i16 / 8);
    }

    fn with_operand<CB>(&mut self, mut callback: CB, addr: Option<u16>)
    where
        CB: FnMut(&mut Self, u16),
    {
        callback(self, addr.expect("Required operand is missing"))
    }
}

impl Instructions for CPU {
    fn adc(&mut self, addr: u16) {
        self.add_to_accumulator(self.memory.read(addr));
    }

    fn and(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator & self.memory.read::<u8>(addr));
    }

    fn asl(&mut self, addr: Option<u16>) {
        let value = match addr {
            Some(addr) => self.memory.read(addr),
            None => self.accumulator,
        };

        let result = value << 1;

        self.status.set(Status::Carry, value >> 7 == 1);
        self.status.set_negative(result);
        self.status.set_zero(result);

        match addr {
            Some(addr) => self.memory.write(addr, result),
            None => self.accumulator = result,
        };
    }

    fn bcc(&mut self, addr: u16) {
        self.branch(addr, !self.status.contains(Status::Carry));
    }

    fn bcs(&mut self, addr: u16) {
        self.branch(addr, self.status.contains(Status::Carry));
    }

    fn beq(&mut self, addr: u16) {
        self.branch(addr, self.status.contains(Status::Zero));
    }

    fn bit(&mut self, addr: u16) {
        let value: u8 = self.memory.read(addr);

        self.status.set_zero(self.accumulator & value);
        self.status.set_overflow(value & 0b0100_0000 != 0);
        self.status.set_negative(value);
    }

    fn bmi(&mut self, addr: u16) {
        self.branch(addr, self.status.contains(Status::Negative));
    }

    fn bne(&mut self, addr: u16) {
        self.branch(addr, !self.status.contains(Status::Zero));
    }

    fn bpl(&mut self, addr: u16) {
        self.branch(addr, !self.status.contains(Status::Negative));
    }

    fn brk(&mut self) {
        self.stack_push(self.program_counter);
        self.stack_push(self.status.bits());

        self.program_counter = self.memory.read(memory::INTERRUPT);

        self.status.set(Status::Break, true);
        self.status.set(Status::Break2, true);
    }

    fn bvc(&mut self, addr: u16) {
        self.branch(addr, !self.status.contains(Status::Overflow));
    }

    fn bvs(&mut self, addr: u16) {
        self.branch(addr, self.status.contains(Status::Overflow));
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

    fn eor(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator ^ self.memory.read::<u8>(addr));
    }

    fn inx(&mut self) {
        self.set_index_x(self.index_x.wrapping_add(1));
    }

    fn lda(&mut self, addr: u16) {
        self.set_accumulator(self.memory.read(addr));
    }

    fn ora(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator | self.memory.read::<u8>(addr));
    }

    fn sbc(&mut self, addr: u16) {
        self.add_to_accumulator(
            (self.memory.read::<u8>(addr) as i8)
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

    fn sta(&mut self, addr: u16) {
        self.memory.write(addr, self.accumulator);
    }

    fn stx(&mut self, addr: u16) {
        self.memory.write(addr, self.index_x);
    }

    fn sty(&mut self, addr: u16) {
        self.memory.write(addr, self.index_y);
    }

    fn tax(&mut self) {
        self.set_index_x(self.accumulator);
    }

    fn tay(&mut self) {
        self.set_index_y(self.accumulator);
    }

    fn tsx(&mut self) {
        self.set_index_x(self.stack_pointer.into());
    }

    fn txa(&mut self) {
        self.set_accumulator(self.index_x);
    }

    fn txs(&mut self) {
        self.stack_pointer = StackPointer(self.index_x);
    }

    fn tya(&mut self) {
        self.set_accumulator(self.index_y);
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
        assert!(cpu.status.contains(Status::Zero));
    }

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xaa, 0x00]);

        assert_eq!(cpu.index_x, 0x0a);
    }

    #[test]
    fn test_0xa8_tay() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0a, 0xa8, 0x00]);

        assert_eq!(cpu.index_y, 0x0a);
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

    #[test]
    fn test_0x85_sta() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0x42, // load 0x42 into the accumulator
            0x85, 0x00, // store the accumulator into $0000
            0x00,
        ]);

        assert_eq!(cpu.memory.read::<u8>(0x00), 0x42)
    }

    #[test]
    fn test_0x86_stx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0x42, // load 0x42 into the accumulator
            0xaa, // transfer the accumulator into X register
            0x86, 0x00, // store X register into $0000
            0x00,
        ]);

        assert_eq!(cpu.memory.read::<u8>(0x00), 0x42);
    }

    #[test]
    fn test_0x84_sty() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9, 0x42, // load 0x42 into the accumulator
            0xa8, // transfer the accumulator into Y register
            0x84, 0x00, // store Y register into $0000
            0x00,
        ]);

        assert_eq!(cpu.memory.read::<u8>(0x00), 0x42);
    }

    #[test]
    fn test_0x0a_asl() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9,
            0b0101_0101, // load 0b0101_0101 into the accumulator
            0x0a,        // accumulator bit shift left
            0x00,
        ]);

        assert!(!cpu.status.contains(Status::Carry));
        assert_eq!(cpu.accumulator, 0b1010_1010);
    }

    #[test]
    fn test_0x0a_asl_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9,
            0b1010_1010, // load 0b1010_1010 into the accumulator
            0x0a,        // accumulator bit shift left
            0x00,
        ]);

        assert!(cpu.status.contains(Status::Carry));
        assert_eq!(cpu.accumulator, 0b0101_0100);
    }

    #[test]
    fn test_0x06_asl() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![
            0xa9,
            0b0101_0101, // load 0b0101_0101 into the accumulator
            0x85,
            0x00, // store the accumulator into $0000
            0x06,
            0x00, // $0000 bit shift left
            0x00,
        ]);

        assert!(!cpu.status.contains(Status::Carry));
        assert_eq!(cpu.memory.read::<u8>(0x00), 0b1010_1010);
    }
}
