use cpu::tests::*;

#[test]
fn test_and_r() {
    macro_rules! test_and_r(
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.a = 0xA3;
            cpu.$r = 0x11;
            let expected = cpu.a & cpu.$r;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.a == expected, format!("and a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert_eq!(cpu.f.z(), expected == 0);
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0xA0;
            cpu.$r = 0x1;
            let expected = cpu.a & cpu.$r;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.a == expected, format!("and a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert_eq!(cpu.f.z(), expected == 0);
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());
        }}
    );
    test_and_r!(b, opcode(0xA0));
    test_and_r!(c, opcode(0xA1));
    test_and_r!(d, opcode(0xA2));
    test_and_r!(e, opcode(0xA3));
    test_and_r!(h, opcode(0xA4));
    test_and_r!(l, opcode(0xA5));
    test_and_r!(a, opcode(0xA7));
}

#[test]
fn test_and_a_deref_hl() {
    let (mut cpu, mut ram) = init(None);
    ram[0x1F01] = 0x11;
    cpu.h = 0x1F;
    cpu.l = 0x1;
    cpu.a = 0xA3;
    let expected = cpu.a & ram[0x1F01];
    test(&mut cpu, &mut ram, 4, opcode(0xA6));
    assert!(
        cpu.a == expected,
        format!(
            "and a, (hl): Expected 0x{:X}, got 0x{:X}",
            expected, ram[0x1F01]
        )
    );
    assert_eq!(cpu.f.z(), expected == 0);
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0xA0;
    ram[0x1F01] = 0x1;
    let expected = cpu.a & ram[0x1F01];
    test(&mut cpu, &mut ram, 4, opcode(0xA6));
    assert!(
        cpu.a == expected,
        format!(
            "and a, (hl): Expected 0x{:X}, got 0x{:X}",
            expected, ram[0x1F01]
        )
    );
    assert_eq!(cpu.f.z(), expected == 0);
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(!cpu.f.c());
}
