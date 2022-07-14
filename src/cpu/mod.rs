use std::fmt::LowerHex;

use self::{
    instructions::{Instruction, Instructions},
    memory::{Memory, MemoryValue},
    mode::Mode,
    opcodes::AddressingMode,
    status::Status,
};

mod cpu_2a03;
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

impl From<StackPointer> for u8 {
    fn from(s: StackPointer) -> Self {
        s.0
    }
}

impl From<StackPointer> for u16 {
    fn from(s: StackPointer) -> Self {
        s.0.into()
    }
}

impl StackPointer {
    /// Advance the stack pointer to the first unoccupied space in the stack.
    pub fn advance(&mut self, offset: i16) {
        self.0 = (i16::from(self.0) + offset) as u8;
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
        let opcodes = &(*opcodes::OPCODES_MAP);

        loop {
            let code: u8 = self.read_program_counter();

            let opcode = opcodes
                .get(&code)
                .unwrap_or_else(|| panic!("Opcode {:x} is not recognized", code));

            let addr = self.get_operand_address(&opcode.mode);

            match opcode.instruction {
                Instruction::Adc => self.with_operand(Self::adc, addr),
                Instruction::And => self.with_operand(Self::and, addr),
                Instruction::Asl => self.asl(addr), // handles None case to operate on accumulator
                Instruction::Bcc => self.with_operand(Self::bcc, addr),
                Instruction::Bcs => self.with_operand(Self::bcs, addr),
                Instruction::Beq => self.with_operand(Self::beq, addr),
                Instruction::Bit => self.with_operand(Self::bit, addr),
                Instruction::Bmi => self.with_operand(Self::bmi, addr),
                Instruction::Bne => self.with_operand(Self::bne, addr),
                Instruction::Bpl => self.with_operand(Self::bpl, addr),
                Instruction::Brk => self.brk(),
                Instruction::Bvc => self.with_operand(Self::bvc, addr),
                Instruction::Bvs => self.with_operand(Self::bvs, addr),
                Instruction::Clc => self.clc(),
                Instruction::Cld => self.cld(),
                Instruction::Cli => self.cli(),
                Instruction::Clv => self.clv(),
                Instruction::Cmp => self.with_operand(Self::cmp, addr),
                Instruction::Cpx => self.with_operand(Self::cpx, addr),
                Instruction::Cpy => self.with_operand(Self::cpy, addr),
                Instruction::Dec => self.with_operand(Self::dec, addr),
                Instruction::Dex => self.dex(),
                Instruction::Dey => self.dey(),
                Instruction::Eor => self.with_operand(Self::eor, addr),
                Instruction::Inc => self.with_operand(Self::inc, addr),
                Instruction::Inx => self.inx(),
                Instruction::Iny => self.iny(),
                Instruction::Jmp => self.with_operand(Self::jmp, addr),
                Instruction::Jsr => self.with_operand(Self::jsr, addr),
                Instruction::Lda => self.with_operand(Self::lda, addr),
                Instruction::Ldx => self.with_operand(Self::ldx, addr),
                Instruction::Ldy => self.with_operand(Self::ldy, addr),
                Instruction::Lsr => todo!(),
                Instruction::Nop => self.nop(),
                Instruction::Ora => self.with_operand(Self::ora, addr),
                Instruction::Pha => todo!(),
                Instruction::Php => todo!(),
                Instruction::Pla => todo!(),
                Instruction::Plp => todo!(),
                Instruction::Rol => self.rol(addr), // handles None case to operate on accumulator
                Instruction::Ror => self.ror(addr), // handles None case to operate on accumulator
                Instruction::Rti => todo!(),
                Instruction::Rts => self.rts(),
                Instruction::Sbc => self.with_operand(Self::sbc, addr),
                Instruction::Sec => self.sec(),
                Instruction::Sed => self.sed(),
                Instruction::Sei => self.sei(),
                Instruction::Sta => self.with_operand(Self::sta, addr),
                Instruction::Stx => self.with_operand(Self::stx, addr),
                Instruction::Sty => self.with_operand(Self::sty, addr),
                Instruction::Tax => self.tax(),
                Instruction::Tay => self.tay(),
                Instruction::Tsx => self.tsx(),
                Instruction::Txa => self.txa(),
                Instruction::Txs => self.txs(),
                Instruction::Tya => self.tya(),
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

            AddressingMode::ZeroPage => Some(self.read_program_counter::<u8>().into()),
            AddressingMode::ZeroPageX => Some(
                self.read_program_counter::<u8>()
                    .wrapping_add(self.index_x)
                    .into(),
            ),
            AddressingMode::ZeroPageY => Some(
                self.read_program_counter::<u8>()
                    .wrapping_add(self.index_y)
                    .into(),
            ),

            AddressingMode::Absolute => Some(self.read_program_counter::<u16>()),
            AddressingMode::AbsoluteX => Some(
                self.read_program_counter::<u16>()
                    .wrapping_add(self.index_x.into()),
            ),
            AddressingMode::AbsoluteY => Some(
                self.read_program_counter::<u16>()
                    .wrapping_add(self.index_y.into()),
            ),

            AddressingMode::Indirect => {
                let addr: u16 = self.read_program_counter();
                // TODO: fail if addr as u8 == 0xff
                Some(self.memory.read(addr))
            }
            AddressingMode::IndirectX => {
                let ptr: u8 = self.read_program_counter();
                Some(self.memory.read(ptr.wrapping_add(self.index_x).into()))
            }
            AddressingMode::IndirectY => {
                let ptr: u8 = self.read_program_counter();
                let addr: u16 = self.memory.read(ptr.into());
                Some(addr.wrapping_add(self.index_y.into()))
            }

            AddressingMode::Relative => {
                let offset = self.read_program_counter::<u8>() as i8;
                Some(self.program_counter.wrapping_add(offset as u16))
            }

            AddressingMode::NoneAddressing => None,
        }
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
