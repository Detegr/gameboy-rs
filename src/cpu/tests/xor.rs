use cpu::tests::*;

#[test]
fn test_xor_r() {
    macro_rules! test_xor_r(
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.a = 0xA3;
            cpu.$r = 0x11;
            let expected = cpu.a ^ cpu.$r;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.a == expected, format!("xor a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert_eq!(cpu.f.z(), expected == 0);
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0xA0;
            cpu.$r = 0xA0;
            let expected = cpu.a ^ cpu.$r;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.a == expected, format!("xor a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert_eq!(cpu.f.z(), expected == 0);
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());
        }}
    );
    test_xor_r!(b, opcode(0xA8));
    test_xor_r!(c, opcode(0xA9));
    test_xor_r!(d, opcode(0xAA));
    test_xor_r!(e, opcode(0xAB));
    test_xor_r!(h, opcode(0xAC));
    test_xor_r!(l, opcode(0xAD));
    test_xor_r!(a, opcode(0xAF));
}

#[test]
fn test_xor_a_deref_hl() {
    let (mut cpu, mut ram) = init(None);
    ram[0x1F01] = 0x11;
    cpu.h = 0x1F;
    cpu.l = 0x1;
    cpu.a = 0xA3;
    let expected = cpu.a ^ ram[0x1F01];
    test(&mut cpu, &mut ram, 8, opcode(0xAE));
    assert!(
        cpu.a == expected,
        format!(
            "xor a, (hl): Expected 0x{:X}, got 0x{:X}",
            expected, ram[0x1F01]
        )
    );
    assert_eq!(cpu.f.z(), expected == 0);
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0xA0;
    ram[0x1F01] = 0xA0;
    let expected = cpu.a ^ ram[0x1F01];
    test(&mut cpu, &mut ram, 8, opcode(0xAE));
    assert!(
        cpu.a == expected,
        format!(
            "xor a, (hl): Expected 0x{:X}, got 0x{:X}",
            expected, ram[0x1F01]
        )
    );
    assert_eq!(cpu.f.z(), expected == 0);
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
}
