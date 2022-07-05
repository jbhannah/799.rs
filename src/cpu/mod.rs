use std::fmt::LowerHex;

use self::{
    instructions::{Instruction, Instructions},
    memory::{Memory, MemoryValue},
    mode::Mode,
    opcodes::AddressingMode,
    status::Status,
};

mod cpu;
mod instructions;
mod memory;
pub mod mode;
mod opcodes;
mod status;

#[cfg(test)]
mod test;

/// One-byte stack pointer.
#[derive(Debug, Clone, Copy)]
pub struct StackPointer(u8);

impl Default for StackPointer {
    /// Default the stack pointer to the first address of the stack in memory.
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
    /// Advance the stack pointer to the first unoccupied space in the stack.
    pub fn advance(&mut self, offset: i16) {
        self.0 = (self.0 as i16 + offset) as u8;
    }
}

/// Implementation of the NES's 6502-like 2A03 CPU.
#[derive(Debug, Default)]
pub struct CPU {
    pub accumulator: u8,
    pub index_x: u8,
    pub index_y: u8,
    pub program_counter: u16,
    pub stack_pointer: StackPointer,
    pub status: Status,
    memory: Memory,
    pub mode: Mode,
}

impl CPU {
    pub fn new() -> Self {
        Default::default()
    }

    /// Load a program into memory, reset the CPU to its initial state, and run
    /// the program.
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    /// Load a program into memory.
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory.load(program, self.mode);
    }

    /// Set the program counter to the value at the designated reset address in
    /// memory, and reset all flags and internal registers to their default
    /// values, while preserving the contents of memory.
    pub fn reset(&mut self) {
        *self = Self {
            program_counter: self.memory.read(memory::RESET),
            memory: self.memory,
            ..Default::default()
        }
    }

    /// Read and execute each instruction in the program.
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
                Instruction::ASL => self.asl(addr), // handles None case to operate on accumulator
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
                Instruction::CMP => self.with_operand(Self::cmp, addr),
                Instruction::CPX => self.with_operand(Self::cpx, addr),
                Instruction::CPY => self.with_operand(Self::cpy, addr),
                Instruction::DEC => self.with_operand(Self::dec, addr),
                Instruction::DEX => self.dex(),
                Instruction::DEY => self.dey(),
                Instruction::EOR => self.with_operand(Self::eor, addr),
                Instruction::INC => self.with_operand(Self::inc, addr),
                Instruction::INX => self.inx(),
                Instruction::INY => self.iny(),
                Instruction::JMP => self.with_operand(Self::jmp, addr),
                Instruction::JSR => self.with_operand(Self::jsr, addr),
                Instruction::LDA => self.with_operand(Self::lda, addr),
                Instruction::LDX => self.with_operand(Self::ldx, addr),
                Instruction::LDY => self.with_operand(Self::ldy, addr),
                Instruction::LSR => todo!(),
                Instruction::NOP => todo!(),
                Instruction::ORA => self.with_operand(Self::ora, addr),
                Instruction::PHA => todo!(),
                Instruction::PHP => todo!(),
                Instruction::PLA => todo!(),
                Instruction::PLP => todo!(),
                Instruction::ROL => self.rol(addr), // handles None case to operate on accumulator
                Instruction::ROR => self.ror(addr), // handles None case to operate on accumulator
                Instruction::RTI => todo!(),
                Instruction::RTS => self.rts(),
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

            // Break if the program counter is empty.
            if self.program_counter == 0 {
                return;
            }
        }
    }

    /// Retrieve an operand address based on the given addressing mode.
    fn get_operand_address(&mut self, mode: &AddressingMode) -> Option<u16> {
        match mode {
            // Return the program counter, and increment it manually since we'll
            // be reading it directly.
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

            AddressingMode::Indirect => {
                let addr: u16 = self.read_program_counter();
                // TODO: fail if addr as u8 == 0xff
                Some(self.memory.read(addr))
            }
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

    /// Read the value at the address of the program counter, and increment the
    /// counter by the number of bytes in the returned value.
    fn read_program_counter<T: MemoryValue + LowerHex>(&mut self) -> T {
        let val: T = self.memory.read(self.program_counter);
        println!("{:x}: {:x}", self.program_counter, val);
        self.program_counter += T::BITS / 8;
        println!("{:x}", self.program_counter);
        val
    }

    /// Call the given callback that requires an operand and panic if the operand
    /// is missing.
    fn with_operand<CB>(&mut self, callback: CB, addr: Option<u16>)
    where
        CB: Fn(&mut Self, u16),
    {
        callback(self, addr.expect("Required operand is missing"))
    }
}
