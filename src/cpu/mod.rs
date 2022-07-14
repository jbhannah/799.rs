use std::fmt::LowerHex;

use self::{
    cpu_6502::Cpu6502,
    instructions::Instructions,
    memory::{Memory, MemoryValue},
    mode::Mode,
    opcodes::AddressingMode,
    status::Status,
};

mod cpu_6502;
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
            self.call(&opcode.instruction, addr);

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

    /// Add the given value to the accumulator.
    fn add_to_accumulator(&mut self, value: u8) {
        let sum =
            u16::from(self.accumulator + value) + u16::from(self.status.contains(Status::Carry));

        self.status.set_carry(sum);

        let result = sum as u8;
        self.status
            .set_overflow((value ^ result) & (result ^ self.accumulator) & 0x80 != 0);

        self.set_accumulator(result);
    }

    /// If the condition is met, add the relative displacement to the program
    /// counter to branch to a new location.
    fn branch(&mut self, displacement: u16, condition: bool) {
        if condition {
            self.program_counter += displacement;
        }
    }

    /// Compare the given value to the value at the given address, and set the
    /// carry, zero, and negative flags accordingly.
    fn compare(&mut self, value: u8, addr: u16) {
        let rhs: u8 = self.memory.read(addr);
        let result = value.wrapping_sub(rhs);

        self.status.set(Status::Carry, value >= rhs);
        self.status.set_zero(result);
        self.status.set_negative(result);
    }

    /// Set the accumulator to the given value and update the negative and zero
    /// status bits.
    fn set_accumulator(&mut self, value: u8) {
        self.accumulator = value;
        self.set_status_negative_zero(value)
    }

    /// Set the X register to the given value and update the negative and zero
    /// status bits.
    fn set_index_x(&mut self, value: u8) {
        self.index_x = value;
        self.set_status_negative_zero(value)
    }

    /// Set the Y register to the given value and update the negative and zero
    /// status bits.
    fn set_index_y(&mut self, value: u8) {
        self.index_y = value;
        self.set_status_negative_zero(value)
    }

    /// Set the negative and zero status bits accordingly for the given value.
    fn set_status_negative_zero(&mut self, value: u8) {
        self.status.set_negative(value);
        self.status.set_zero(value);
    }

    /// Pop a value off of the stack and advance the stack pointer in reverse.
    /// TODO: prevent pop on empty stack
    /// TODO: reset popped addresses to 0
    fn stack_pop<T: MemoryValue>(&mut self) -> T {
        self.stack_pointer.advance(T::BITS as i16 / -8);
        let val: T = self.memory.read(self.stack_pointer.into());
        val
    }

    /// Push a value onto the stack and advance the stack pointer.
    /// TODO: prevent push past limit of stack
    fn stack_push<T: MemoryValue>(&mut self, value: T) {
        self.memory.write(self.stack_pointer.into(), value);
        self.stack_pointer.advance(T::BITS as i16 / 8);
    }
}

impl Cpu6502 for CPU {
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
        self.set_status_negative_zero(result);

        match addr {
            Some(addr) => self.memory.write(addr, result),
            None => self.accumulator = result,
        };
    }

    fn bcc(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Carry));
    }

    fn bcs(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Carry));
    }

    fn beq(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Zero));
    }

    fn bit(&mut self, addr: u16) {
        let value: u8 = self.memory.read(addr);

        self.status.set_zero(self.accumulator & value);
        self.status.set_overflow(value & 0b0100_0000 != 0);
        self.status.set_negative(value);
    }

    fn bmi(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Negative));
    }

    fn bne(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Zero));
    }

    fn bpl(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Negative));
    }

    fn brk(&mut self) {
        self.stack_push(self.program_counter);
        self.stack_push(self.status.bits());

        self.program_counter = self.memory.read(memory::INTERRUPT);

        self.status.set(Status::Break, true);
        self.status.set(Status::Break2, true);
    }

    fn bvc(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Overflow));
    }

    fn bvs(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Overflow));
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

    fn cmp(&mut self, addr: u16) {
        self.compare(self.accumulator, addr);
    }

    fn cpx(&mut self, addr: u16) {
        self.compare(self.index_x, addr);
    }

    fn cpy(&mut self, addr: u16) {
        self.compare(self.index_y, addr);
    }

    fn dec(&mut self, addr: u16) {
        let value: u8 = self.memory.read(addr);
        let result = value.wrapping_sub(1);

        self.memory.write(addr, result);

        self.set_status_negative_zero(result);
    }

    fn dex(&mut self) {
        self.set_index_x(self.index_x.wrapping_sub(1));
    }

    fn dey(&mut self) {
        self.set_index_y(self.index_y.wrapping_sub(1));
    }

    fn eor(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator ^ self.memory.read::<u8>(addr));
    }

    fn inc(&mut self, addr: u16) {
        let value: u8 = self.memory.read(addr);
        let result = value.wrapping_add(1);

        self.memory.write(addr, result);

        self.set_status_negative_zero(result);
    }

    fn inx(&mut self) {
        self.set_index_x(self.index_x.wrapping_add(1));
    }

    fn iny(&mut self) {
        self.set_index_y(self.index_y.wrapping_add(1));
    }

    fn jmp(&mut self, addr: u16) {
        self.program_counter = addr;
    }

    fn jsr(&mut self, addr: u16) {
        self.stack_push(self.program_counter + 1);
        self.program_counter = addr;
    }

    fn lda(&mut self, addr: u16) {
        self.set_accumulator(self.memory.read(addr));
    }

    fn ldx(&mut self, addr: u16) {
        self.set_index_x(self.memory.read(addr));
    }

    fn ldy(&mut self, addr: u16) {
        self.set_index_y(self.memory.read(addr));
    }

    fn lsr(&mut self, addr: Option<u16>) {
        let initial = match addr {
            Some(addr) => self.memory.read(addr),
            None => self.accumulator,
        };

        let result = initial >> 1;

        self.status.set(Status::Carry, initial % 2 == 1);
        self.set_status_negative_zero(result);

        match addr {
            Some(addr) => self.memory.write(addr, result),
            None => self.accumulator = result,
        }
    }

    fn ora(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator | self.memory.read::<u8>(addr));
    }

    fn rol(&mut self, addr: Option<u16>) {
        let initial = match addr {
            Some(addr) => self.memory.read(addr),
            None => self.accumulator,
        };

        let result = (initial << 1) | self.status.and(Status::Carry).bits();

        self.status.set(Status::Carry, initial >> 7 == 1);
        self.set_status_negative_zero(result);

        match addr {
            Some(addr) => self.memory.write(addr, result),
            None => self.accumulator = result,
        };
    }

    fn ror(&mut self, addr: Option<u16>) {
        let initial = match addr {
            Some(addr) => self.memory.read(addr),
            None => self.accumulator,
        };

        let result = (initial >> 1) | (self.status.and(Status::Carry).bits() << 7);

        self.status.set(Status::Carry, initial & 1 == 1);
        self.set_status_negative_zero(result);

        match addr {
            Some(addr) => self.memory.write(addr, result),
            None => self.accumulator = result,
        };
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop();
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
