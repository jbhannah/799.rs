use super::*;

#[test]
fn test_0xa0_ldy_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0x00]);

    assert_eq!(cpu.index_y, 0x05);
}

#[test]
fn test_0xa2_ldx_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa2, 0x05, 0x00]);

    assert_eq!(cpu.index_x, 0x05);
}

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
fn test_0xe6_inc_zero_page() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x10, 0x55 as u8);
    cpu.load_and_run(vec![0xe6, 0x10, 0x00]);

    assert_eq!(cpu.memory.read::<u8>(0x10), 0x56);
}

#[test]
fn test_0xe8_inx_overflow() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xe8, 0xe8, 0x00]);
    cpu.reset();
    cpu.index_x = 0xff;
    cpu.run();

    assert_eq!(cpu.index_x, 1);
}

#[test]
fn test_0xe8_iny_overflow() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xc8, 0xc8, 0x00]);
    cpu.reset();
    cpu.index_y = 0xff;
    cpu.run();

    assert_eq!(cpu.index_y, 1);
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

#[test]
fn test_0xc6_dec_absolute() {
    let mut cpu = CPU::new();
    cpu.memory.write(0x1010, 0x42 as u8);
    cpu.load_and_run(vec![0xce, 0x10, 0x10, 0x00]);

    assert_eq!(cpu.memory.read::<u8>(0x1010), 0x41);
}

#[test]
fn test_0xca_dex() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xca, 0x00]);
    cpu.reset();
    cpu.index_x = 0x42;
    cpu.run();

    assert_eq!(cpu.index_x, 0x41);
}

#[test]
fn test_0x88_dey() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x88, 0x00]);
    cpu.reset();
    cpu.index_y = 0x42;
    cpu.run();

    assert_eq!(cpu.index_y, 0x41);
}
