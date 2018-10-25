use cpu::tests::*;

#[test]
fn test_jr_n() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0x18);
    mmu.write_u8(cpu.pc + 1, 0x5);

    let expected = cpu.pc + 2 + 5;
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.pc = 0xFFFE;
    mmu.write_u8(cpu.pc, 0x18);
    mmu.write_u8(cpu.pc + 1, 0x1);
    let expected = 1;
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    mmu.write_u8(cpu.pc, 0x18);
    mmu.write_u8(cpu.pc + 1, -5i8 as u8);

    let expected = cpu.pc + 2 - 5;
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);
}

#[test]
fn test_jr_cc_n() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0x20);
    mmu.write_u8(cpu.pc + 1, 0x5);

    let expected = cpu.pc + 2;

    cpu.f.set_z();
    let expected_cycles = cpu.cycles + 8;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    cpu.f.unset_z();
    let expected = cpu.pc + 2 + 5;
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    cpu.f.unset_z();
    mmu.write_u8(cpu.pc, 0x28);
    let expected = cpu.pc + 2;
    let expected_cycles = cpu.cycles + 8;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    cpu.f.set_z();
    let expected = cpu.pc + 2 + 5;
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();

    mmu.write_u8(cpu.pc, 0x30);
    mmu.write_u8(cpu.pc + 1, 0x5);

    let expected = cpu.pc + 2;

    cpu.f.set_c();
    let expected_cycles = cpu.cycles + 8;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    cpu.f.unset_c();
    let expected = cpu.pc + 2 + 5;
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    cpu.f.unset_c();
    mmu.write_u8(cpu.pc, 0x38);
    let expected = cpu.pc + 2;
    let expected_cycles = cpu.cycles + 8;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    cpu.f.set_c();
    let expected = cpu.pc + 2 + 5;
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, expected);
}
