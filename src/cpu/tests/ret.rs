use cpu;
use cpu::tests::*;

#[test]
fn test_ret() {
    let (mut cpu, mut mmu) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    mmu.write_u8(cpu.sp + 1, 0xC0);
    mmu.write_u8(cpu.sp + 2, 0xAA);

    test(&mut cpu, &mut mmu, 16, opcode(0xC9));

    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_reti() {
    let (mut cpu, mut mmu) = init(None);
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    cpu.interrupts = cpu::InterruptState::Disabled;

    mmu.write_u8(cpu.sp + 1, 0xC0);
    mmu.write_u8(cpu.sp + 2, 0xAA);

    test(&mut cpu, &mut mmu, 16, opcode(0xD9));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
    assert_eq!(cpu.interrupts, cpu::InterruptState::Enabled);
}

#[test]
fn test_ret_nz() {
    let (mut cpu, mut mmu) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    mmu.write_u8(cpu.sp + 1, 0xC0);
    mmu.write_u8(cpu.sp + 2, 0xAA);

    cpu.f.set_z();

    test(&mut cpu, &mut mmu, 8, opcode(0xC0));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.unset_z();

    test(&mut cpu, &mut mmu, 20, opcode(0xC0));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_ret_z() {
    let (mut cpu, mut mmu) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    mmu.write_u8(cpu.sp + 1, 0xC0);
    mmu.write_u8(cpu.sp + 2, 0xAA);

    cpu.f.unset_z();

    test(&mut cpu, &mut mmu, 8, opcode(0xC8));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.set_z();

    test(&mut cpu, &mut mmu, 20, opcode(0xC8));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_ret_nc() {
    let (mut cpu, mut mmu) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    mmu.write_u8(cpu.sp + 1, 0xC0);
    mmu.write_u8(cpu.sp + 2, 0xAA);

    cpu.f.set_c();

    test(&mut cpu, &mut mmu, 8, opcode(0xD0));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.unset_c();

    test(&mut cpu, &mut mmu, 20, opcode(0xD0));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_ret_c() {
    let (mut cpu, mut mmu) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    mmu.write_u8(cpu.sp + 1, 0xC0);
    mmu.write_u8(cpu.sp + 2, 0xAA);

    cpu.f.unset_c();

    test(&mut cpu, &mut mmu, 8, opcode(0xD8));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.set_c();

    test(&mut cpu, &mut mmu, 20, opcode(0xD8));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}
