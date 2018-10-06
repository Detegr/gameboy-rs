use cpu::tests::*;

#[test]
fn test_jr_n() {
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();

    ram[cpu.pc as usize] = 0x1;

    let expected = cpu.pc + 1;
    test(&mut cpu, &mut ram, 12, opcode(0x18));
    assert_eq!(cpu.pc, expected);

    cpu.pc = 0xFFFF;
    ram[cpu.pc as usize] = 0x1;
    let expected = 0;
    test(&mut cpu, &mut ram, 12, opcode(0x18));
    assert_eq!(cpu.pc, expected);

    cpu.reset();
    ram[cpu.pc as usize] = -1i8 as u8;

    let expected = cpu.pc - 1;
    test(&mut cpu, &mut ram, 12, opcode(0x18));
    assert_eq!(cpu.pc, expected);
}

#[test]
fn test_jr_cc_n() {
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();

    ram[cpu.pc as usize] = 0x1;
    let unchanged = cpu.pc;
    let expected = cpu.pc + 1;

    cpu.f.set_z();
    assert!(cpu.f.z());
    test(&mut cpu, &mut ram, 8, opcode(0x20));
    assert_eq!(cpu.pc, unchanged);

    cpu.f.unset_z();
    test(&mut cpu, &mut ram, 12, opcode(0x20));
    assert_eq!(cpu.pc, expected);

    cpu.f.unset_z();
    ram[cpu.pc as usize] = 0x1;
    let unchanged = cpu.pc;
    test(&mut cpu, &mut ram, 8, opcode(0x28));
    assert_eq!(cpu.pc, unchanged);

    cpu.f.set_z();
    let expected = cpu.pc + 1;
    test(&mut cpu, &mut ram, 12, opcode(0x28));
    assert_eq!(cpu.pc, expected);

    cpu.reset();

    ram[cpu.pc as usize] = 0x1;
    let unchanged = cpu.pc;
    let expected = cpu.pc + 1;

    cpu.f.set_c();
    assert!(cpu.f.c());
    test(&mut cpu, &mut ram, 8, opcode(0x30));
    assert_eq!(cpu.pc, unchanged);

    cpu.f.unset_c();
    test(&mut cpu, &mut ram, 12, opcode(0x30));
    assert_eq!(cpu.pc, expected);

    cpu.f.unset_c();
    ram[cpu.pc as usize] = 0x1;
    let unchanged = cpu.pc;
    test(&mut cpu, &mut ram, 8, opcode(0x38));
    assert_eq!(cpu.pc, unchanged);

    cpu.f.set_c();
    let expected = cpu.pc + 1;
    test(&mut cpu, &mut ram, 12, opcode(0x38));
    assert_eq!(cpu.pc, expected);
}
