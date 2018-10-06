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
