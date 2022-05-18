#[derive(Debug)]
pub enum Instruction {
    ADC, // ADd with Carry
    AND, // logical AND
    ASL, // Arithmetic Shift Left
    BCC, // Branch if Carry Clear
    BCS, // Branch if Carry Set
    BEQ, // Branch if EQual
    BIT, // BIT test
    BMI, // Branch if MInus
    BNE, // Branch if Not Equal
    BPL, // Branch if PLus
    BRK, // BReaK (force interrupt)
    BVC, // Branch if oVerflow Clear
    BVS, // Branch if oVerflow Set
    CLC, // CLear Carry flag
    CLD, // CLear Decimal flag
    CLI, // CLear Interrupt disable flag
    CLV, // CLear oVerflow flag
    CMP, // CoMPare
    CPX, // ComPare X register
    CPY, // ComPare Y register
    DEC, // DECrecment memory
    DEX, // DEcrement X register
    DEY, // DEcrement Y register
    EOR, // Exclusive OR
    INC, // INCrement memory
    INX, // INcrement X register
    INY, // INcrement Y register
    JMP, // JuMP
    JSR, // Jump to SubRoutine
    LDA, // LoaD Accumulator
    LDX, // LoaD X register
    LDY, // LoaD Y register
    LSR, // Logical Shift Right
    NOP, // No OPeration
    ORA, // logical inclusive OR on Accumulator
    PHA, // PusH Accumulator
    PHP, // PusH Processor status
    PLA, // PuLl Accumulator
    PLP, // PuLl Processor status
    ROL, // ROtate Left
    ROR, // ROtate Right
    RTI, // ReTurn from Interrupt
    RTS, // ReTurn from Subroutine
    SBC, // SuBtract with Carry
    SEC, // SEt Carry flag
    SED, // SEt Decimal flag
    SEI, // SEt Interrupt disable flag
    STA, // STore Accumulator
    STX, // STore X register
    STY, // STore Y register
    TAX, // Transfer Accumulator to X register
    TAY, // Transfer Accumulator to Y register
    TSX, // Transfer Stack pointer to X register
    TXA, // Transfer X to Accumulator
    TXS, // Transfer X to Stack pointer
    TYA, // Transfer Y to Accumulator
}

pub trait Instructions {
    fn adc(&mut self, operand: u16);
    fn inx(&mut self);
    fn lda(&mut self, operand: u16);
    fn sbc(&mut self, operand: u16);
    fn tax(&mut self);
}
