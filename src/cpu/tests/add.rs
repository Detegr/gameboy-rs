use cpu::tests::*;

#[test]
fn test_add_a_r() {
    macro_rules! test_add_a_r {
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.a = 0x10;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "add a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x1F;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "add a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0xFF;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "add a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());

            cpu.a = 0xF0;
            cpu.$r = 0x11;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "add a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(cpu.f.c());

            cpu.a = 0x10;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "add a, {}: Expected 0x{:X}, got 0x{:X}",
                    stringify!($r),
                    expected,
                    cpu.a
                )
            );
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());
        }};
    }
    fn test_add_a_a() {
        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x4;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(
            cpu.a == expected,
            format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x8;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(
            cpu.a == expected,
            format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x0;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(
            cpu.a == expected,
            format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0xF0;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(
            cpu.a == expected,
            format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());
    }

    test_add_a_a();
    test_add_a_r!(b, opcode(0x80));
    test_add_a_r!(c, opcode(0x81));
    test_add_a_r!(d, opcode(0x82));
    test_add_a_r!(e, opcode(0x83));
    test_add_a_r!(h, opcode(0x84));
    test_add_a_r!(l, opcode(0x85));
    fn test_add_a_hl() {
        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x10;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(ram[cpu.hl()]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(
            cpu.a == expected,
            format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x1F;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(ram[cpu.hl()]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(
            cpu.a == expected,
            format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0xFF;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(ram[cpu.hl()]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(
            cpu.a == expected,
            format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x11;
        cpu.a = 0xF0;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(ram[cpu.hl()]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(
            cpu.a == expected,
            format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());
    }
    test_add_a_hl();
    fn test_add_a_n() {
        let (mut cpu, mut ram) = init(None);
        cpu.reset();

        let val = 0x1;
        cpu.a = 0x10;
        ram[cpu.pc] = val;
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut ram, 8, opcode(0xC6));
        assert!(
            cpu.a == expected,
            format!(
                "add a, {}: Expected 0x{:X}, got 0x{:X}",
                stringify!($r),
                expected,
                cpu.a
            )
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        cpu.reset();
        cpu.a = 0x1F;
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut ram, 8, opcode(0xC6));
        assert!(
            cpu.a == expected,
            format!(
                "add a, {}: Expected 0x{:X}, got 0x{:X}",
                stringify!($r),
                expected,
                cpu.a
            )
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        cpu.reset();
        cpu.a = 0xFF;
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut ram, 8, opcode(0xC6));
        assert!(
            cpu.a == expected,
            format!(
                "add a, {}: Expected 0x{:X}, got 0x{:X}",
                stringify!($r),
                expected,
                cpu.a
            )
        );
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        cpu.reset();
        let val = 0x11;
        cpu.a = 0xF0;
        ram[cpu.pc] = val;
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut ram, 8, opcode(0xC6));
        assert!(
            cpu.a == expected,
            format!(
                "add a, {}: Expected 0x{:X}, got 0x{:X}",
                stringify!($r),
                expected,
                cpu.a
            )
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        cpu.reset();
        let val = 0x1;
        cpu.a = 0x10;
        ram[cpu.pc] = val;
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut ram, 8, opcode(0xC6));
        assert!(
            cpu.a == expected,
            format!(
                "add a, {}: Expected 0x{:X}, got 0x{:X}",
                stringify!($r),
                expected,
                cpu.a
            )
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    }
    test_add_a_n();
}

#[test]
fn test_add_rr_rr() {
    macro_rules! test_add_rr_rr {
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
            let expected = 0xFFFF_u16.wrapping_add(0x0001);
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
    test_add_rr_rr!(h, l, b, c, opcode(0x9));
    test_add_rr_rr!(h, l, d, e, opcode(0x19));
    // test_add_rr_rr!(h, l, h, l, opcode(0x29));
    // test_add_rr_rr!(h, l, h, l, opcode(0x39));
}

#[test]
fn test_add_hl_hl() {
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
    let expected = 0xF0F0_u16.wrapping_add(0xF0F0);
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

#[test]
fn test_add_hl_sp() {
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
    let expected = 0xF0F0_u16.wrapping_add(0xF0F0);
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

#[test]
fn test_add_sp_n() {
    let (mut cpu, mut ram) = init(None);
    cpu.reset();
    cpu.sp = 0x1000;
    ram[cpu.pc] = 0x13;
    test(&mut cpu, &mut ram, 16, opcode(0xE8));
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
    assert_eq!(cpu.sp, 0x1013);

    cpu.sp = 0x10FF;
    ram[cpu.pc] = 0x1;
    test(&mut cpu, &mut ram, 16, opcode(0xE8));
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(cpu.f.c());
    assert_eq!(cpu.sp, 0x1100);

    cpu.sp = 0x100F;
    ram[cpu.pc] = 0x1;
    test(&mut cpu, &mut ram, 16, opcode(0xE8));
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(!cpu.f.c());
    assert_eq!(cpu.sp, 0x1010);

    cpu.sp = 0x1000;
    ram[cpu.pc] = -1i8 as u8;
    test(&mut cpu, &mut ram, 16, opcode(0xE8));
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(cpu.f.c());
    assert_eq!(cpu.sp, 0x0FFF);

    cpu.sp = 0x10F0;
    ram[cpu.pc] = -1i8 as u8;
    test(&mut cpu, &mut ram, 16, opcode(0xE8));
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(cpu.f.h());
    assert!(!cpu.f.c());
    assert_eq!(cpu.sp, 0x10EF);
}
