use super::cpu_6502::Cpu6502;

/// Values representing each discrete instruction supported by the 6502 CPU.
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
    fn call(&mut self, instruction: &Instruction, addr: Option<u16>);
    fn with_operand<CB>(&mut self, callback: CB, addr: Option<u16>)
    where
        CB: Fn(&mut Self, u16);
}

impl<T> Instructions for T
where
    T: Cpu6502,
{
    /// Call the corresponding function for the given instruction.
    fn call(&mut self, instruction: &Instruction, addr: Option<u16>) {
        match instruction {
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
            Instruction::Lsr => self.lsr(addr), // handles None case to operate on accumulator
            Instruction::Nop => self.nop(),
            Instruction::Ora => self.with_operand(Self::ora, addr),
            Instruction::Pha => self.pha(),
            Instruction::Php => self.php(),
            Instruction::Pla => self.pla(),
            Instruction::Plp => self.plp(),
            Instruction::Rol => self.rol(addr), // handles None case to operate on accumulator
            Instruction::Ror => self.ror(addr), // handles None case to operate on accumulator
            Instruction::Rti => self.rti(),
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
