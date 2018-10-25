use cpu::tests::*;

#[test]
fn test_or_r() {
    macro_rules! test_or_r(
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut mmu) = init(None);
            cpu.a = 0xA3;
            cpu.$r = 0x11;
            let expected = cpu.a | cpu.$r;
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(cpu.a == expected, format!("or a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert_eq!(cpu.f.z(), expected == 0);
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x0;
            cpu.$r = 0x0;
            let expected = cpu.a | cpu.$r;
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(cpu.a == expected, format!("or a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert_eq!(cpu.f.z(), expected == 0);
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());
        }}
    );
    test_or_r!(b, opcode(0xB0));
    test_or_r!(c, opcode(0xB1));
    test_or_r!(d, opcode(0xB2));
    test_or_r!(e, opcode(0xB3));
    test_or_r!(h, opcode(0xB4));
    test_or_r!(l, opcode(0xB5));
    test_or_r!(a, opcode(0xB7));
}

#[test]
fn test_or_a_deref_hl() {
    let (mut cpu, mut mmu) = init(None);
    mmu.write_u8(0x1F01, 0x11);
    cpu.h = 0x1F;
    cpu.l = 0x1;
    cpu.a = 0xA3;
    let expected = cpu.a | mmu.read_u8(0x1F01);
    test(&mut cpu, &mut mmu, 8, opcode(0xB6));
    assert!(
        cpu.a == expected,
        format!(
            "or a, (hl): Expected 0x{:X}, got 0x{:X}",
            expected,
            mmu.read_u8(0x1F01)
        )
    );
    assert_eq!(cpu.f.z(), expected == 0);
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0x0;
    mmu.write_u8(0x1F01, 0x0);
    let expected = cpu.a | mmu.read_u8(0x1F01);
    test(&mut cpu, &mut mmu, 8, opcode(0xB6));
    assert!(
        cpu.a == expected,
        format!(
            "or a, (hl): Expected 0x{:X}, got 0x{:X}",
            expected,
            mmu.read_u8(0x1F01)
        )
    );
    assert_eq!(cpu.f.z(), expected == 0);
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
}
