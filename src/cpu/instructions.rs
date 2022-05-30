#[derive(Debug)]
pub enum Instruction {
    /// ADd with Carry
    ADC,
    /// logical AND
    AND,
    /// Arithmetic Shift Left
    ASL,
    /// Branch if Carry Clear
    BCC,
    /// Branch if Carry Set
    BCS,
    /// Branch if EQual
    BEQ,
    /// BIT test
    BIT,
    /// Branch if MInus
    BMI,
    /// Branch if Not Equal
    BNE,
    /// Branch if PLus
    BPL,
    /// BReaK (force interrupt)
    BRK,
    /// Branch if oVerflow Clear
    BVC,
    /// Branch if oVerflow Set
    BVS,
    /// CLear Carry flag
    CLC,
    /// CLear Decimal flag
    CLD,
    /// CLear Interrupt disable flag
    CLI,
    /// CLear oVerflow flag
    CLV,
    /// CoMPare
    CMP,
    /// ComPare X register
    CPX,
    /// ComPare Y register
    CPY,
    /// DECrecment memory
    DEC,
    /// DEcrement X register
    DEX,
    /// DEcrement Y register
    DEY,
    /// Exclusive OR
    EOR,
    /// INCrement memory
    INC,
    /// INcrement X register
    INX,
    /// INcrement Y register
    INY,
    /// JuMP
    JMP,
    /// Jump to SubRoutine
    JSR,
    /// LoaD Accumulator
    LDA,
    /// LoaD X register
    LDX,
    /// LoaD Y register
    LDY,
    /// Logical Shift Right
    LSR,
    /// No OPeration
    NOP,
    /// logical inclusive OR on Accumulator
    ORA,
    /// PusH Accumulator
    PHA,
    /// PusH Processor status
    PHP,
    /// PuLl Accumulator
    PLA,
    /// PuLl Processor status
    PLP,
    /// ROtate Left
    ROL,
    /// ROtate Right
    ROR,
    /// ReTurn from Interrupt
    RTI,
    /// ReTurn from Subroutine
    RTS,
    /// SuBtract with Carry
    SBC,
    /// SEt Carry flag
    SEC,
    /// SEt Decimal flag
    SED,
    /// SEt Interrupt disable flag
    SEI,
    /// STore Accumulator
    STA,
    /// STore X register
    STX,
    /// STore Y register
    STY,
    /// Transfer Accumulator to X register
    TAX,
    /// Transfer Accumulator to Y register
    TAY,
    /// Transfer Stack pointer to X register
    TSX,
    /// Transfer X to Accumulator
    TXA,
    /// Transfer X to Stack pointer
    TXS,
    /// Transfer Y to Accumulator
    TYA,
}

pub trait Instructions {
    fn adc(&mut self, addr: u16);
    fn and(&mut self, addr: u16);
    fn asl(&mut self, addr: Option<u16>);
    fn bcc(&mut self, addr: u16);
    fn bcs(&mut self, addr: u16);
    fn beq(&mut self, addr: u16);
    fn bit(&mut self, addr: u16);
    fn bmi(&mut self, addr: u16);
    fn bne(&mut self, addr: u16);
    fn bpl(&mut self, addr: u16);
    fn brk(&mut self);
    fn bvc(&mut self, addr: u16);
    fn bvs(&mut self, addr: u16);
    fn clc(&mut self);
    fn cld(&mut self);
    fn cli(&mut self);
    fn clv(&mut self);
    fn eor(&mut self, addr: u16);
    fn inx(&mut self);
    fn lda(&mut self, addr: u16);
    fn ora(&mut self, addr: u16);
    fn sbc(&mut self, addr: u16);
    fn sec(&mut self);
    fn sed(&mut self);
    fn sei(&mut self);
    fn sta(&mut self, addr: u16);
    fn stx(&mut self, addr: u16);
    fn sty(&mut self, addr: u16);
    fn tax(&mut self);
    fn tay(&mut self);
    fn tsx(&mut self);
    fn txa(&mut self);
    fn txs(&mut self);
    fn tya(&mut self);
}
