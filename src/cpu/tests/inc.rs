use cpu::tests::*;

#[test]
fn test_inc_r() {
    macro_rules! test_inc_r(
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut mmu) = init(None);
            cpu.$r = 0x11;
            let expected = cpu.$r.wrapping_add(0x1);
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(cpu.$r == expected, format!("inc {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());

            cpu.$r = 0xF;
            let expected = cpu.$r.wrapping_add(0x1);
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(cpu.$r == expected, format!("inc {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());

            cpu.$r = 0xFF;
            let expected = cpu.$r.wrapping_add(0x1);
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(cpu.$r == expected, format!("inc {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());

            cpu.$r = 0x11;
            let expected = cpu.$r.wrapping_add(0x1);
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(cpu.$r == expected, format!("inc {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
        }}
    );
    test_inc_r!(a, opcode(0x3C));
    test_inc_r!(b, opcode(0x4));
    test_inc_r!(c, opcode(0xC));
    test_inc_r!(d, opcode(0x14));
    test_inc_r!(e, opcode(0x1C));
    test_inc_r!(h, opcode(0x24));
    test_inc_r!(l, opcode(0x2C));
    fn test_inc_hl() {
        let (mut cpu, mut mmu) = init(None);
        mmu.write_u8(0x1F01, 0x15);
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = mmu.read_u8(0x1F01) + 1;
        test(&mut cpu, &mut mmu, 12, opcode(0x34));
        assert!(
            mmu.read_u8(cpu.hl()) == expected,
            format!(
                "inc (hl): Expected 0x{:X}, got 0x{:X}",
                expected,
                mmu.read_u8(cpu.hl())
            )
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());

        let (mut cpu, mut mmu) = init(None);
        mmu.write_u8(0x1F01, 0x1F);
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = mmu.read_u8(0x1F01) + 1;
        test(&mut cpu, &mut mmu, 12, opcode(0x34));
        assert!(
            mmu.read_u8(cpu.hl()) == expected,
            format!(
                "inc (hl): Expected 0x{:X}, got 0x{:X}",
                expected,
                mmu.read_u8(cpu.hl())
            )
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());

        let (mut cpu, mut mmu) = init(None);
        mmu.write_u8(0x1F01, 0xFF);
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = mmu.read_u8(0x1F01).wrapping_add(1);
        test(&mut cpu, &mut mmu, 12, opcode(0x34));
        assert!(
            mmu.read_u8(cpu.hl()) == expected,
            format!(
                "inc (hl): Expected 0x{:X}, got 0x{:X}",
                expected,
                mmu.read_u8(cpu.hl())
            )
        );
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
    }
    test_inc_hl();
}

#[test]
fn test_inc_rr() {
    macro_rules! test_inc_rr(
        ($r1:ident, $r2:ident, $r1_val:expr, $r2_val:expr, $expected_r1:expr, $expected_r2:expr, $cycles:expr, $func:expr) => {{
            let (mut cpu, mut mmu) = init(None);
            cpu.$r1 = $r1_val;
            cpu.$r2 = $r2_val;
            test(&mut cpu, &mut mmu, $cycles, $func);
            assert!(cpu.$r1 == $expected_r1, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), $expected_r1, cpu.$r1));
            assert!(cpu.$r2 == $expected_r2, format!("inc {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), $expected_r2, cpu.$r2));
        }}
    );
    test_inc_rr!(b, c, 0x0, 0x0, 0x0, 0x1, 8, opcode(0x03));
    test_inc_rr!(d, e, 0x2, 0xFF, 0x3, 0x0, 8, opcode(0x13));
    test_inc_rr!(h, l, 0xFF, 0xFF, 0x0, 0x0, 8, opcode(0x23));
}

#[test]
fn test_inc_sp() {
    let (mut cpu, mut mmu) = init(None);
    cpu.sp = 0xFFFE;
    test(&mut cpu, &mut mmu, 8, opcode(0x33));
    assert!(cpu.sp == 0xFFFF);
    test(&mut cpu, &mut mmu, 8, opcode(0x33));
    assert!(cpu.sp == 0x0);
    test(&mut cpu, &mut mmu, 8, opcode(0x33));
    assert!(cpu.sp == 0x1);
}
