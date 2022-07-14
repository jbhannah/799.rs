#[derive(Debug)]
pub enum Instruction {
    /// ADd with Carry
    Adc,
    /// logical AND
    And,
    /// Arithmetic Shift Left
    Asl,
    /// Branch if Carry Clear
    Bcc,
    /// Branch if Carry Set
    Bcs,
    /// Branch if EQual
    Beq,
    /// BIT test
    Bit,
    /// Branch if MInus
    Bmi,
    /// Branch if Not Equal
    Bne,
    /// Branch if PLus
    Bpl,
    /// BReaK (force interrupt)
    Brk,
    /// Branch if oVerflow Clear
    Bvc,
    /// Branch if oVerflow Set
    Bvs,
    /// CLear Carry flag
    Clc,
    /// CLear Decimal flag
    Cld,
    /// CLear Interrupt disable flag
    Cli,
    /// CLear oVerflow flag
    Clv,
    /// CoMPare
    Cmp,
    /// ComPare X register
    Cpx,
    /// ComPare Y register
    Cpy,
    /// DECrecment memory
    Dec,
    /// DEcrement X register
    Dex,
    /// DEcrement Y register
    Dey,
    /// Exclusive OR
    Eor,
    /// INCrement memory
    Inc,
    /// INcrement X register
    Inx,
    /// INcrement Y register
    Iny,
    /// JuMP
    Jmp,
    /// Jump to SubRoutine
    Jsr,
    /// LoaD Accumulator
    Lda,
    /// LoaD X register
    Ldx,
    /// LoaD Y register
    Ldy,
    /// Logical Shift Right
    Lsr,
    /// No OPeration
    Nop,
    /// logical inclusive OR on Accumulator
    Ora,
    /// PusH Accumulator
    Pha,
    /// PusH Processor status
    Php,
    /// PuLl Accumulator
    Pla,
    /// PuLl Processor status
    Plp,
    /// ROtate Left
    Rol,
    /// ROtate Right
    Ror,
    /// ReTurn from Interrupt
    Rti,
    /// ReTurn from Subroutine
    Rts,
    /// SuBtract with Carry
    Sbc,
    /// SEt Carry flag
    Sec,
    /// SEt Decimal flag
    Sed,
    /// SEt Interrupt disable flag
    Sei,
    /// STore Accumulator
    Sta,
    /// STore X register
    Stx,
    /// STore Y register
    Sty,
    /// Transfer Accumulator to X register
    Tax,
    /// Transfer Accumulator to Y register
    Tay,
    /// Transfer Stack pointer to X register
    Tsx,
    /// Transfer X to Accumulator
    Txa,
    /// Transfer X to Stack pointer
    Txs,
    /// Transfer Y to Accumulator
    Tya,
}

pub trait Instructions {
    /// Add the accumulator, the value at the given address, and the carry bit,
    /// and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if the result overflows past bit 7.
    /// * Z - set if the result is zero.
    /// * V - set if bit 7 of the result is incorrect.
    /// * N - set if bit 7 of the result is set.
    fn adc(&mut self, addr: u16);

    /// Perform a bitwise and between the accumulator and the value at the given
    /// address, and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is zero.
    /// * N - set if bit 7 of the result is set.
    fn and(&mut self, addr: u16);

    /// Shift the accumulator or the value at the given address to the left by 1
    /// and store the result in the same location.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if bit 7 of the initial value is set.
    /// * Z - set if the result is zero.
    /// * N - set if bit 7 of the result is set.
    fn asl(&mut self, addr: Option<u16>);

    /// Branch to the given address if the carry bit is not set.
    fn bcc(&mut self, displacement: u16);

    /// Branch to the given address if the carry bit is set.
    fn bcs(&mut self, displacement: u16);

    /// Branch to the given address if the zero bit is set.
    fn beq(&mut self, displacement: u16);

    /// Perform a bit test on the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the bitwise and of the accumulator and the value is zero.
    /// * V - set to bit 6 of the value.
    /// * N - set to bit 7 of the value.
    fn bit(&mut self, addr: u16);

    /// Branch to the given address if the negative bit is set.
    fn bmi(&mut self, displacement: u16);

    /// Branch to the given address if the zero bit is not set.
    fn bne(&mut self, displacement: u16);

    /// Branch to the given address if the negative bit is not set.
    fn bpl(&mut self, displacement: u16);

    /// Force an interrupt, pushing the the program counter and processor status
    /// onto the stack, setting the program counter to the value at the
    /// designated interrupt address, and set the break bits.
    ///
    /// Processor status bits affected:
    ///
    /// * B - set to 1.
    /// * B2 - set to 1.
    fn brk(&mut self);

    /// Branch to the given address if the overflow bit is not set.
    fn bvc(&mut self, displacement: u16);

    /// Branch to the given address if the overflow bit is set.
    fn bvs(&mut self, displacement: u16);

    /// Clear the carry bit.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set to 0.
    fn clc(&mut self);

    /// Clear the decimal bit.
    ///
    /// Processor status bits affected:
    ///
    /// * D - set to 0.
    fn cld(&mut self);

    /// Clear the interrupt disable bit.
    ///
    /// Processor status bits affected:
    ///
    /// * I - set to 0.
    fn cli(&mut self);

    /// Clear the overflow bit.
    ///
    /// Processor status bits affected:
    ///
    /// * V - set to 0.
    fn clv(&mut self);

    /// Compare the value at the given address to the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if accumulator >= value at address.
    /// * Z - set if accumulator == value at address.
    /// * N - set if accumulator <= value at address.
    fn cmp(&mut self, addr: u16);

    /// Compare the value at the given address to the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if X register >= value at address.
    /// * Z - set if X register == value at address.
    /// * N - set if X register <= value at address.
    fn cpx(&mut self, addr: u16);

    /// Compare the value at the given address to the Y register.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if Y register >= value at address.
    /// * Z - set if Y register == value at address.
    /// * N - set if Y register <= value at address.
    fn cpy(&mut self, addr: u16);

    /// Decrement the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn dec(&mut self, addr: u16);

    /// Decrement the value in the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn dex(&mut self);

    /// Decrement the value in the Y register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn dey(&mut self);

    /// Perform a bitwise exclusive or between the accumulator and the value at
    /// the given address, and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn eor(&mut self, addr: u16);

    /// Increment the value at the given address by 1, wrapping to 0 if the
    /// result would overflow.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn inc(&mut self, addr: u16);

    /// Increment the X register by 1, wrapping to 0 if the result would
    /// overflow.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn inx(&mut self);

    /// Increment the Y register by 1, wrapping to 0 if the result would
    /// overflow.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn iny(&mut self);

    /// Set the program counter to the specified address.
    fn jmp(&mut self, addr: u16);

    /// Push the address of the next sequential instruction onto the stack, and
    /// set the program counter to the given address.
    fn jsr(&mut self, addr: u16);

    /// Set the accumulator to the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn lda(&mut self, addr: u16);

    /// Set the X register to the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn ldx(&mut self, addr: u16);

    /// Set the Y register to the value at the given address.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn ldy(&mut self, addr: u16);

    /// Perform a bitwise or between the accumulator and the value at the given
    /// address, and store the result in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn ora(&mut self, addr: u16);

    /// Rotate the bits in the accumulator or at the given address one place to
    /// the left through the carry bit.
    ///
    /// Processor status bits affected:
    /// * C - set to bit 7 of the initial value.
    /// * Z - set if the result is 0.
    /// * N - set if the bit 7 of the new value is set.
    fn rol(&mut self, addr: Option<u16>);

    /// Rotate the bits in the accumulator or at the given address one place to
    /// the right through the carry bit.
    ///
    /// Processor status bits affected:
    /// * C - set to bit 0 of the initial value.
    /// * Z - set if the result is 0.
    /// * N - set if the bit 7 of the new value is set.
    fn ror(&mut self, addr: Option<u16>);

    /// Return from a subroutine by setting the program counter to the last
    /// value on the stack.
    fn rts(&mut self);

    /// Subtract the value at the given address from the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set if the result overflows past bit 7.
    /// * Z - set if the result is 0.
    /// * V - set if bit 7 of the result is incorrect.
    /// * N - set to bit 7 of the result.
    fn sbc(&mut self, addr: u16);

    /// Set the carry bit.
    ///
    /// Processor status bits affected:
    ///
    /// * C - set to 1.
    fn sec(&mut self);

    /// Set the decimal bit.
    ///
    /// Processor status bits affected:
    ///
    /// * D - set to 1.
    fn sed(&mut self);

    /// Set the interrupt disable bit.
    ///
    /// Processor status bits affected:
    ///
    /// * I - set to 1.
    fn sei(&mut self);

    /// Store the accumulator at the given address.
    fn sta(&mut self, addr: u16);

    /// Store the X register at the given address.
    fn stx(&mut self, addr: u16);

    /// Store the Y register at the given address.
    fn sty(&mut self, addr: u16);

    /// Store the accumulator in the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tax(&mut self);

    /// Store the accumulator in the Y register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tay(&mut self);

    /// Store the stack pointer in the X register.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tsx(&mut self);

    /// Store the X register in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn txa(&mut self);

    /// Store the X register in the stack pointer.
    fn txs(&mut self);

    /// Store the Y register in the accumulator.
    ///
    /// Processor status bits affected:
    ///
    /// * Z - set if the result is 0.
    /// * N - set to bit 7 of the result.
    fn tya(&mut self);
}
