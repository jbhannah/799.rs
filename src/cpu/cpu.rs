use super::{
    instructions::Instructions,
    memory::{self, MemoryValue},
    status::Status,
    StackPointer, CPU,
};

impl CPU {
    /// Add the given value to the accumulator.
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
    /// Add the accumulator, the value at the given address, and the carry bit,
    /// and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if the result overflows past bit 7.
    /// * Z - set if the result is zero.
    /// * V - set if bit 7 of the result is incorrect.
    /// * N - set if bit 7 of the result is set.
    fn adc(&mut self, addr: u16) {
        self.add_to_accumulator(self.memory.read(addr));
    }

    /// Perform a bitwise and between the accumulator and the value at the given
    /// address, and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is zero.
    /// * N - set if bit 7 of the result is set.
    fn and(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator & self.memory.read::<u8>(addr));
    }

    /// Shift the accumulator or the value at the given address to the left by 1
    /// and store the result in the same location.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if bit 7 of the initial value is set.
    /// * Z - set if the result is zero.
    /// * N - set if bit 7 of the result is set.
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

    /// Branch to the given address if the carry bit is not set.
    fn bcc(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Carry));
    }

    /// Branch to the given address if the carry bit is set.
    fn bcs(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Carry));
    }

    /// Branch to the given address if the zero bit is set.
    fn beq(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Zero));
    }

    /// Perform a bit test on the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the bitwise and of the accumulator and the value is zero.
    /// * V - set to bit 6 of the value.
    /// * N - set to bit 7 of the value.
    fn bit(&mut self, addr: u16) {
        let value: u8 = self.memory.read(addr);

        self.status.set_zero(self.accumulator & value);
        self.status.set_overflow(value & 0b0100_0000 != 0);
        self.status.set_negative(value);
    }

    /// Branch to the given address if the negative bit is set.
    fn bmi(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Negative));
    }

    /// Branch to the given address if the zero bit is not set.
    fn bne(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Zero));
    }

    /// Branch to the given address if the negative bit is not set.
    fn bpl(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Negative));
    }

    /// Force an interrupt, pushing the the program counter and processor status
    /// onto the stack, setting the program counter to the value at the
    /// designated interrupt address, and set the break bits.
    ///
    /// Processor status bits affected:
    ///
    /// * B - set to 1.
    /// * B2 - set to 1.
    fn brk(&mut self) {
        self.stack_push(self.program_counter);
        self.stack_push(self.status.bits());

        self.program_counter = self.memory.read(memory::INTERRUPT);

        self.status.set(Status::Break, true);
        self.status.set(Status::Break2, true);
    }

    /// Branch to the given address if the overflow bit is not set.
    fn bvc(&mut self, displacement: u16) {
        self.branch(displacement, !self.status.contains(Status::Overflow));
    }

    /// Branch to the given address if the overflow bit is set.
    fn bvs(&mut self, displacement: u16) {
        self.branch(displacement, self.status.contains(Status::Overflow));
    }

    /// Clear the carry bit.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set to 0.
    fn clc(&mut self) {
        self.status.set(Status::Carry, false);
    }

    /// Clear the decimal bit.
    ///
    /// Processor status bits affected:
    ///
    /// * D - set to 0.
    fn cld(&mut self) {
        self.status.set(Status::Decimal, false);
    }

    /// Clear the interrupt disable bit.
    ///
    /// Processor status bits affected:
    ///
    /// * I - set to 0.
    fn cli(&mut self) {
        self.status.set(Status::InterruptDisable, false);
    }

    /// Clear the overflow bit.
    ///
    /// Processor status bits affected:
    ///
    /// * V - set to 0.
    fn clv(&mut self) {
        self.status.set(Status::Overflow, false);
    }

    /// Compare the value at the given address to the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if accumulator >= value at address.
    /// * Z - set if accumulator == value at address.
    /// * N - set if accumulator <= value at address.
    fn cmp(&mut self, addr: u16) {
        self.compare(self.accumulator, addr);
    }

    /// Compare the value at the given address to the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if X register >= value at address.
    /// * Z - set if X register == value at address.
    /// * N - set if X register <= value at address.
    fn cpx(&mut self, addr: u16) {
        self.compare(self.index_x, addr);
    }

    /// Compare the value at the given address to the Y register.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if Y register >= value at address.
    /// * Z - set if Y register == value at address.
    /// * N - set if Y register <= value at address.
    fn cpy(&mut self, addr: u16) {
        self.compare(self.index_y, addr);
    }

    /// Decrement the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn dec(&mut self, addr: u16) {
        let value: u8 = self.memory.read(addr);
        let result = value.wrapping_sub(1);

        self.memory.write(addr, result);

        self.status.set_zero(result);
        self.status.set_negative(result);
    }

    /// Decrement the value in the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn dex(&mut self) {
        self.set_index_x(self.index_x.wrapping_sub(1));
    }

    /// Decrement the value in the Y register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn dey(&mut self) {
        self.set_index_y(self.index_y.wrapping_sub(1));
    }

    /// Perform a bitwise exclusive or between the accumulator and the value at
    /// the given address, and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn eor(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator ^ self.memory.read::<u8>(addr));
    }

    /// Increment the X register by 1, wrapping to 0 if the result would
    /// overflow.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn inx(&mut self) {
        self.set_index_x(self.index_x.wrapping_add(1));
    }

    /// Push the address of the next sequential instruction onto the stack, and
    /// set the program counter to the given address.
    fn jsr(&mut self, addr: u16) {
        self.stack_push(self.program_counter + 1);
        self.program_counter = addr;
    }

    /// Set the accumulator to the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn lda(&mut self, addr: u16) {
        self.set_accumulator(self.memory.read(addr));
    }

    /// Set the X register to the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn ldx(&mut self, addr: u16) {
        self.set_index_x(self.memory.read(addr));
    }

    /// Set the Y register to the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn ldy(&mut self, addr: u16) {
        self.set_index_y(self.memory.read(addr));
    }

    /// Perform a bitwise or between the accumulator and the value at the given
    /// address, and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn ora(&mut self, addr: u16) {
        self.set_accumulator(self.accumulator | self.memory.read::<u8>(addr));
    }

    /// Return from a subroutine by setting the program counter to the last
    /// value on the stack.
    fn rts(&mut self) {
        self.program_counter = self.stack_pop();
    }

    /// Subtract the value at the given address from the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if the result overflows past bit 7.
    /// * Z - set if the result is 0.
    /// * V - set if bit 7 of the result is incorrect.
    /// * N - set to bit 7 of the result.
    fn sbc(&mut self, addr: u16) {
        self.add_to_accumulator(
            (self.memory.read::<u8>(addr) as i8)
                .wrapping_neg()
                .wrapping_sub(1) as u8,
        );
    }

    /// Set the carry bit.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set to 1.
    fn sec(&mut self) {
        self.status.set(Status::Carry, true);
    }

    /// Set the decimal bit.
    ///
    /// Processor status bits affected:
    ///
    /// * D - set to 1.
    fn sed(&mut self) {
        self.status.set(Status::Decimal, true);
    }

    /// Set the interrupt disable bit.
    ///
    /// Processor status bits affected:
    ///
    /// * I - set to 1.
    fn sei(&mut self) {
        self.status.set(Status::InterruptDisable, true);
    }

    /// Store the accumulator at the given address.
    fn sta(&mut self, addr: u16) {
        self.memory.write(addr, self.accumulator);
    }

    /// Store the X register at the given address.
    fn stx(&mut self, addr: u16) {
        self.memory.write(addr, self.index_x);
    }

    /// Store the Y register at the given address.
    fn sty(&mut self, addr: u16) {
        self.memory.write(addr, self.index_y);
    }

    /// Store the accumulator in the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tax(&mut self) {
        self.set_index_x(self.accumulator);
    }

    /// Store the accumulator in the Y register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tay(&mut self) {
        self.set_index_y(self.accumulator);
    }

    /// Store the stack pointer in the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tsx(&mut self) {
        self.set_index_x(self.stack_pointer.into());
    }

    /// Store the X register in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn txa(&mut self) {
        self.set_accumulator(self.index_x);
    }

    /// Store the X register in the stack pointer.
    fn txs(&mut self) {
        self.stack_pointer = StackPointer(self.index_x);
    }

    /// Store the Y register in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tya(&mut self) {
        self.set_accumulator(self.index_y);
    }
}
