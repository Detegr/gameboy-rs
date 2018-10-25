use cpu::tests::*;

#[test]
fn test_call() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xCD);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    let old_cycles = cpu.cycles;
    cpu.step(&mut mmu);
    assert_eq!(cpu.cycles, old_cycles + 24);
    assert_eq!(cpu.pc, 0x1122);
    assert_eq!(mmu.read_u8(cpu.sp + 1), (0x103_u16 & 0xFF) as u8);
    assert_eq!(mmu.read_u8(cpu.sp + 2), ((0x103_u16 & 0xFF00) >> 8) as u8);
}

#[test]
fn test_call_nz() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xC4);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    cpu.f.set_z();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 12);
    assert_eq!(cpu.pc, 0x103);
    assert_eq!(cpu.sp, 0xFFFE);

    cpu.reset();
    cpu.f.unset_z();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 24);
    assert_eq!(cpu.pc, 0x1122);
    assert_ne!(cpu.sp, 0xFFFE);
    assert_eq!(mmu.read_u8(cpu.sp + 1), (0x103_u16 & 0xFF) as u8);
    assert_eq!(mmu.read_u8(cpu.sp + 2), ((0x103_u16 & 0xFF00) >> 8) as u8);
}

#[test]
fn test_call_z() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xCC);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    cpu.f.unset_z();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 12);
    assert_eq!(cpu.pc, 0x103);
    assert_eq!(cpu.sp, 0xFFFE);

    cpu.reset();
    cpu.f.set_z();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 24);
    assert_eq!(cpu.pc, 0x1122);
    assert_ne!(cpu.sp, 0xFFFE);
    assert_eq!(mmu.read_u8(cpu.sp + 1), (0x103_u16 & 0xFF) as u8);
    assert_eq!(mmu.read_u8(cpu.sp + 2), ((0x103_u16 & 0xFF00) >> 8) as u8);
}

#[test]
fn test_call_nc() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xD4);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    cpu.f.set_c();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 12);
    assert_eq!(cpu.pc, 0x103);
    assert_eq!(cpu.sp, 0xFFFE);

    cpu.reset();
    cpu.f.unset_c();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 24);
    assert_eq!(cpu.pc, 0x1122);
    assert_ne!(cpu.sp, 0xFFFE);
    assert_eq!(mmu.read_u8(cpu.sp + 1), (0x103_u16 & 0xFF) as u8);
    assert_eq!(mmu.read_u8(cpu.sp + 2), ((0x103_u16 & 0xFF00) >> 8) as u8);
}

#[test]
fn test_call_c() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();

    mmu.write_u8(cpu.pc, 0xDC);
    mmu.write_u8(cpu.pc + 1, 0x22);
    mmu.write_u8(cpu.pc + 2, 0x11);

    cpu.f.unset_c();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 12);
    assert_eq!(cpu.pc, 0x103);
    assert_eq!(cpu.sp, 0xFFFE);

    cpu.reset();
    cpu.f.set_c();

    let old_cycles = cpu.cycles;
    assert_eq!(cpu.sp, 0xFFFE);
    cpu.step(&mut mmu);

    assert_eq!(cpu.cycles, old_cycles + 24);
    assert_eq!(cpu.pc, 0x1122);
    assert_ne!(cpu.sp, 0xFFFE);
    assert_eq!(mmu.read_u8(cpu.sp + 1), (0x103_u16 & 0xFF) as u8);
    assert_eq!(mmu.read_u8(cpu.sp + 2), ((0x103_u16 & 0xFF00) >> 8) as u8);
}
