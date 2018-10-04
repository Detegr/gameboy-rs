use cpu::tests::*;

#[test]
fn test_sub_a_r() {
    macro_rules! test_sub_a_r {
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.a = 0x10;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_sub(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "sub a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x1F;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_sub(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "sub a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x0;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_sub(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "sub a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());

            cpu.a = 0xF0;
            cpu.$r = 0x11;
            let expected = cpu.a.wrapping_sub(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "sub a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x10;
            cpu.$r = 0x10;
            let expected = cpu.a.wrapping_sub(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "sub a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(cpu.f.z());
            assert!(cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());
        }};
    }
    fn test_sub_a_a() {
        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x4;
        let expected = cpu.a.wrapping_sub(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x97));
        assert!(
            cpu.a == expected,
            format!("sub a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x0;
        let expected = cpu.a.wrapping_sub(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x97));
        assert!(
            cpu.a == expected,
            format!("sub a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    }

    test_sub_a_a();
    test_sub_a_r!(b, opcode(0x90));
    test_sub_a_r!(c, opcode(0x91));
    test_sub_a_r!(d, opcode(0x92));
    test_sub_a_r!(e, opcode(0x93));
    test_sub_a_r!(h, opcode(0x94));
    test_sub_a_r!(l, opcode(0x95));
    fn test_sub_a_hl() {
        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x10;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_sub(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x96));
        assert!(
            cpu.a == expected,
            format!("sub a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x1F;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_sub(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x96));
        assert!(
            cpu.a == expected,
            format!("sub a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x0;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_sub(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x96));
        assert!(
            cpu.a == expected,
            format!("sub a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x11;
        cpu.a = 0x11;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_sub(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x96));
        assert!(
            cpu.a == expected,
            format!("sub a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    }
    test_sub_a_hl();
}

/*
#[test]
fn test_sub_rr_rr() {
    macro_rules! test_sub_rr_rr {
        ($r1:ident, $r2:ident, $r3:ident, $r4:ident, $func:expr) => {
            let (mut cpu, mut ram) = init(None);
            cpu.$r1 = 0x01;
            cpu.$r2 = 0x01;
            cpu.$r3 = 0x01;
            cpu.$r4 = 0x10;
            let expected = 0x0101 + 0x0110;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            let r1r2 = ((cpu.$r1 as u16) << 8) | cpu.$r2 as u16;
            assert_eq!(r1r2, expected);

            cpu.$r1 = 0x0F;
            cpu.$r2 = 0x01;
            cpu.$r3 = 0x01;
            cpu.$r4 = 0x10;
            let expected = 0x0F01 + 0x0110;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            let r1r2 = ((cpu.$r1 as u16) << 8) | cpu.$r2 as u16;
            assert_eq!(r1r2, expected);

            cpu.$r1 = 0xFF;
            cpu.$r2 = 0xFF;
            cpu.$r3 = 0x00;
            cpu.$r4 = 0x01;
            let expected = 0xFFFF_u16.wrapping_sub(0x0001);
            test(&mut cpu, &mut ram, 8, $func);
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());

            let r1r2 = ((cpu.$r1 as u16) << 8) | cpu.$r2 as u16;
            assert_eq!(r1r2, expected);

            cpu.$r1 = 0x01;
            cpu.$r2 = 0x01;
            cpu.$r3 = 0x01;
            cpu.$r4 = 0x10;
            let expected = 0x0101 + 0x0110;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            let r1r2 = ((cpu.$r1 as u16) << 8) | cpu.$r2 as u16;
            assert_eq!(r1r2, expected);
        };
    }
    test_sub_rr_rr!(h, l, b, c, opcode(0x9));
    test_sub_rr_rr!(h, l, d, e, opcode(0x19));
}
*/

/*
#[test]
fn test_sub_hl_hl() {
    let (mut cpu, mut ram) = init(None);
    cpu.h = 0x2;
    cpu.l = 0x2;
    let expected = 0x0202 + 0x0202;
    test(&mut cpu, &mut ram, 8, opcode(0x29));
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);

    cpu.h = 0x0F;
    cpu.l = 0x01;
    let expected = 0x0F01 + 0x0F01;
    test(&mut cpu, &mut ram, 8, opcode(0x29));
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(!cpu.f.c());
    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);

    cpu.h = 0xF0;
    cpu.l = 0xF0;
    let expected = 0xF0F0_u16.wrapping_sub(0xF0F0);
    test(&mut cpu, &mut ram, 8, opcode(0x29));
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(cpu.f.c());

    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);

    cpu.h = 0x2;
    cpu.l = 0x2;
    let expected = 0x0202 + 0x0202;
    test(&mut cpu, &mut ram, 8, opcode(0x29));
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);
}
*/

/*
#[test]
fn test_sub_hl_sp() {
    let (mut cpu, mut ram) = init(None);
    cpu.reset();
    cpu.h = 0x2;
    cpu.l = 0x2;
    cpu.sp = 0x202;
    let expected = cpu.sp + 0x202;
    test(&mut cpu, &mut ram, 8, opcode(0x39));
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);

    cpu.h = 0x0F;
    cpu.l = 0x01;
    cpu.sp = 0x101;
    let expected = 0xF01 + 0x101;
    test(&mut cpu, &mut ram, 8, opcode(0x39));
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(!cpu.f.c());

    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);

    cpu.h = 0xF0;
    cpu.l = 0xF0;
    cpu.sp = 0xF0F0;
    let expected = 0xF0F0_u16.wrapping_sub(0xF0F0);
    test(&mut cpu, &mut ram, 8, opcode(0x39));
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(cpu.f.c());

    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);

    cpu.h = 0x2;
    cpu.l = 0x2;
    cpu.sp = 0x202;
    let expected = 0x0202 + 0x0202;
    test(&mut cpu, &mut ram, 8, opcode(0x39));
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    let r1r2 = ((cpu.h as u16) << 8) | cpu.l as u16;
    assert_eq!(r1r2, expected);
}
*/
