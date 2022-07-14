use super::*;

#[test]
fn test_0x00_brk() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x00]);
    cpu.reset();

    let status = cpu.status.bits();
    let pc = cpu.program_counter;

    cpu.run();

    assert_eq!(cpu.stack_pop::<u8>(), status);
    assert_eq!(cpu.stack_pop::<u16>(), pc + 1);

    assert!(cpu.status.contains(Status::Break));
    assert!(cpu.status.contains(Status::Break2));
}

#[test]
fn test_0xa0_ldy_immediate() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa0, 0x05, 0x00]);

    assert_eq!(cpu.index_y, 0x05);
}

#[test]
fn test_0xa1_lda_indirect_x() {
    let mut cpu = CPU::new();

    cpu.load(vec![0xa1, 0x20, 0x00]);
    cpu.reset();

    cpu.index_x = 0x10;
    let addr: u16 = 0xbafc;

    cpu.memory.write((0x20 + cpu.index_x).into(), addr);
    cpu.memory.write(addr, 0x42_u8);

    cpu.run();

    assert_eq!(cpu.accumulator, 0x42);
}

#[test]
fn test_0xb1_lda_indirect_y() {
    let mut cpu = CPU::new();

    cpu.load(vec![0xb1, 0x20, 0x00]);
    cpu.reset();

    cpu.index_y = 0x10;
    let addr: u16 = 0xbafc;

    cpu.memory.write(0x20, addr);
    cpu.memory.write(addr + u16::from(cpu.index_y), 0x42_u8);

    cpu.run();

    assert_eq!(cpu.accumulator, 0x42);
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
    cpu.memory.write(0x10, 0x55_u8);
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
    cpu.memory.write(0x10, 0x55_u8);
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
    cpu.status |= Status::Carry;
    cpu.load_and_run(vec![0x18, 0x00]);

    assert!(!cpu.status.contains(Status::Carry));
}

#[test]
fn test_0xd8_cld() {
    let mut cpu = CPU::new();
    cpu.status |= Status::Decimal;
    cpu.load_and_run(vec![0xd8, 0x00]);

    assert!(!cpu.status.contains(Status::Decimal));
}

#[test]
fn test_0x58_cli() {
    let mut cpu = CPU::new();
    cpu.status |= Status::InterruptDisable;
    cpu.load_and_run(vec![0x58, 0x00]);

    assert!(!cpu.status.contains(Status::InterruptDisable));
}

#[test]
fn test_0xb8_clv() {
    let mut cpu = CPU::new();
    cpu.status |= Status::Overflow;
    cpu.load_and_run(vec![0xb8, 0x00]);

    assert!(!cpu.status.contains(Status::Overflow));
}

#[test]
fn test_0x38_sec() {
    let mut cpu = CPU::new();
    cpu.status &= Status::Carry.not();
    cpu.load_and_run(vec![0x38, 0x00]);

    assert!(cpu.status.contains(Status::Carry));
}

#[test]
fn test_0xf8_sed() {
    let mut cpu = CPU::new();
    cpu.status &= Status::Decimal.not();
    cpu.load_and_run(vec![0xf8, 0x00]);

    assert!(cpu.status.contains(Status::Decimal));
}

#[test]
fn test_0x78_sei() {
    let mut cpu = CPU::new();
    cpu.status &= Status::InterruptDisable.not();
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
    cpu.memory.write(0x1010, 0x42_u8);
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

#[test]
fn test_0x2a_rol_accumulator() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0b1010_1010, 0x2a, 0x00]);

    assert_eq!(cpu.accumulator, 0b0101_0100);
    assert!(cpu.status.contains(Status::Carry));
}

#[test]
fn test_0x6a_ror_accumulator() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0xa9, 0b0101_0101, 0x6a, 0x00]);

    assert_eq!(cpu.accumulator, 0b0010_1010);
    assert!(cpu.status.contains(Status::Carry));
}

#[test]
fn test_0x6c_jmp_indirect() {
    let mut cpu = CPU::new();
    let addr: u16 = 0xbafc;

    cpu.memory.write(0x0120, addr); // set the value at $0120 and $0121 to the address of the next instruction
    cpu.memory.write(addr, 0x42a9_u16); // load 0x42 into the accumulator (0xa9, 0x42 stored little-endian)
    cpu.memory.write(addr + 2, 0x00_u8);

    cpu.load_and_run(vec![0x6c, 0x20, 0x01, 0x00]);

    assert_eq!(cpu.accumulator, 0x42);
}

#[test]
fn test_0x4a_lsr_accumulator() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x4a, 0x00]);
    cpu.reset();

    cpu.accumulator = 0b0101_0101;

    cpu.run();

    assert_eq!(cpu.accumulator, 0b0010_1010);
    assert!(cpu.status.contains(Status::Carry));
}

#[test]
fn test_0x48_pha() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x48, 0x00]);
    cpu.reset();

    cpu.accumulator = 0x42;

    cpu.run();

    assert_eq!(
        cpu.memory.read::<u8>(StackPointer::default().into()),
        cpu.accumulator
    );
}

#[test]
fn test_0x08_php() {
    let mut cpu = CPU::new();
    cpu.load_and_run(vec![0x08, 0x00]);

    assert_eq!(
        cpu.memory.read::<u8>(StackPointer::default().into()),
        cpu.status.bits()
    );
}

#[test]
fn test_0x68_pla() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x68, 0x00]);
    cpu.reset();

    let val = 0x42_u8;
    assert_ne!(cpu.accumulator, val);

    cpu.stack_push(val);
    cpu.run();

    assert_eq!(cpu.accumulator, val);
}

#[test]
fn test_0x28_plp() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x28, 0x00]);
    cpu.reset();

    let val = 0xff_u8;
    assert_ne!(cpu.status.bits(), val);

    cpu.stack_push(val);
    cpu.run();

    assert_eq!(cpu.status.bits(), val);
}

#[test]
fn test_0x40_rti() {
    let mut cpu = CPU::new();
    cpu.load(vec![0x40, 0x00]);
    cpu.reset();

    let pc = 0xa000_u16;
    cpu.stack_push(pc);

    let status = 0xff_u8;
    cpu.stack_push(status);

    cpu.memory.write(pc, 0x00_u8);

    cpu.run();

    assert_eq!(cpu.stack_pop::<u8>(), status);
    assert_eq!(cpu.stack_pop::<u16>(), pc + 1);
    // assert_eq!(
    //     cpu.memory
    //         .read::<u16>(cpu.stack_pointer.wrapping_add(1).into()),
    //     pc
    // );
    // assert_eq!(cpu.status.bits(), status);
}
