use cpu::tests::*;

#[test]
fn test_jp() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0x22);
    mmu.write_u8(cpu.pc + 1, 0x11);

    assert_ne!(cpu.pc, 0x1122);
    test(&mut cpu, &mut mmu, 16, opcode(0xC3));
    assert_eq!(cpu.pc, 0x1122);
}

#[test]
fn test_jp_nz() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xC2);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    assert_eq!(cpu.pc, 0x100);
    cpu.f.set_z();
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x103);

    cpu.reset();
    cpu.f.unset_z();
    let expected_cycles = cpu.cycles + 16;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x1122);
}

#[test]
fn test_jp_z() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xCA);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    assert_eq!(cpu.pc, 0x100);
    cpu.f.unset_z();
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x103);

    cpu.reset();
    cpu.f.set_z();
    let expected_cycles = cpu.cycles + 16;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x1122);
}

#[test]
fn test_jp_nc() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xD2);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    assert_eq!(cpu.pc, 0x100);
    cpu.f.set_c();
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x103);

    cpu.reset();
    cpu.f.unset_c();
    let expected_cycles = cpu.cycles + 16;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x1122);
}

#[test]
fn test_jp_c() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xDA);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    assert_eq!(cpu.pc, 0x100);
    cpu.f.unset_c();
    let expected_cycles = cpu.cycles + 12;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x103);

    cpu.reset();
    cpu.f.set_c();
    let expected_cycles = cpu.cycles + 16;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, expected_cycles);
    assert_eq!(cpu.pc, 0x1122);
}

#[test]
fn test_jp_hl() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    cpu.l = 0x22;
    cpu.h = 0x11;

    assert_ne!(cpu.pc, 0x1122);
    test(&mut cpu, &mut mmu, 4, opcode(0xE9));
    assert_eq!(cpu.pc, 0x1122);
}
