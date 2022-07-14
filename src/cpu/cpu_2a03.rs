use super::{
    instructions::Instructions,
    memory::{self, MemoryValue},
    status::Status,
    StackPointer, CPU,
};

impl CPU {
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
