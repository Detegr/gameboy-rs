use cpu::tests::*;

macro_rules! test_push_r_r {
    ($r1:ident, $r2:ident, $func:expr) => {
        let (mut cpu, mut ram) = init(None);
        cpu.reset();

        cpu.$r1 = 0x12;
        cpu.$r2 = 0x34;

        test(&mut cpu, &mut ram, 16, $func);

        assert_eq!(ram[cpu.sp + 1], 0x34);
        assert_eq!(ram[cpu.sp + 2], 0x12);
        assert_eq!(cpu.sp, 0xFFFE - 2);
    };
}

#[test]
fn test_push() {
    test_push_r_r!(b, c, opcode(0xC5));
    test_push_r_r!(d, e, opcode(0xD5));
    test_push_r_r!(h, l, opcode(0xE5));

    let (mut cpu, mut ram) = init(None);
    cpu.reset();

    cpu.a = 0x12;
    cpu.f.0 = 0x34;

    test(&mut cpu, &mut ram, 16, opcode(0xF5));

    assert_eq!(ram[cpu.sp + 1], 0x34);
    assert_eq!(ram[cpu.sp + 2], 0x12);
    assert_eq!(cpu.sp, 0xFFFE - 2);
}
