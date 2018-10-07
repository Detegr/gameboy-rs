use cpu::tests::*;

#[test]
fn test_ret() {
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    ram[(cpu.sp) as usize] = 0xC0;
    ram[(cpu.sp + 1) as usize] = 0xAA;

    test(&mut cpu, &mut ram, 16, opcode(0xC9));

    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_ret_nz() {
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    ram[(cpu.sp) as usize] = 0xC0;
    ram[(cpu.sp + 1) as usize] = 0xAA;

    cpu.f.set_z();

    test(&mut cpu, &mut ram, 8, opcode(0xC0));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.unset_z();

    test(&mut cpu, &mut ram, 20, opcode(0xC0));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_ret_z() {
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    ram[(cpu.sp) as usize] = 0xC0;
    ram[(cpu.sp + 1) as usize] = 0xAA;

    cpu.f.unset_z();

    test(&mut cpu, &mut ram, 8, opcode(0xC8));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.set_z();

    test(&mut cpu, &mut ram, 20, opcode(0xC8));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_ret_nc() {
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    ram[(cpu.sp) as usize] = 0xC0;
    ram[(cpu.sp + 1) as usize] = 0xAA;

    cpu.f.set_c();

    test(&mut cpu, &mut ram, 8, opcode(0xD0));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.unset_c();

    test(&mut cpu, &mut ram, 20, opcode(0xD0));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}

#[test]
fn test_ret_c() {
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();
    let old_sp = cpu.sp;
    cpu.sp -= 2;
    ram[(cpu.sp) as usize] = 0xC0;
    ram[(cpu.sp + 1) as usize] = 0xAA;

    cpu.f.unset_c();

    test(&mut cpu, &mut ram, 8, opcode(0xD8));
    assert_ne!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0x100);

    cpu.f.set_c();

    test(&mut cpu, &mut ram, 20, opcode(0xD8));
    assert_eq!(cpu.sp, old_sp);
    assert_eq!(cpu.pc, 0xAAC0);
}
