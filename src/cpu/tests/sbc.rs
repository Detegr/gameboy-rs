use cpu::tests::*;

#[test]
fn test_sbc_a_r() {
    macro_rules! test_sbc_a_r {
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.a = 0x10;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_sub(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "sbc a, {}: Expected 0x{:X}, got 0x{:X}",
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
                    "sbc a, {}: Expected 0x{:X}, got 0x{:X}",
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
                    "sbc a, {}: Expected 0x{:X}, got 0x{:X}",
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
            let carry = 1;
            let expected = cpu.a.wrapping_sub(cpu.$r).wrapping_sub(carry);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "sbc a, {}: Expected 0x{:X}, got 0x{:X}",
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
                    "sbc a, {}: Expected 0x{:X}, got 0x{:X}",
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
    fn test_sbc_a_a() {
        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x4;
        let expected = cpu.a.wrapping_sub(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x9F));
        assert!(
            cpu.a == expected,
            format!("sbc a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x0;
        let expected = cpu.a.wrapping_sub(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x9F));
        assert!(
            cpu.a == expected,
            format!("sbc a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    }
    test_sbc_a_a();
    test_sbc_a_r!(b, opcode(0x98));
    test_sbc_a_r!(c, opcode(0x99));
    test_sbc_a_r!(d, opcode(0x9A));
    test_sbc_a_r!(e, opcode(0x9B));
    test_sbc_a_r!(h, opcode(0x9C));
    test_sbc_a_r!(l, opcode(0x9D));

    fn test_sbc_a_hl() {
        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x10;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_sub(ram[cpu.hl()]);
        test(&mut cpu, &mut ram, 8, opcode(0x9E));
        assert!(
            cpu.a == expected,
            format!("sbc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
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
        let expected = cpu.a.wrapping_sub(ram[cpu.hl()]);
        test(&mut cpu, &mut ram, 8, opcode(0x9E));
        assert!(
            cpu.a == expected,
            format!("sbc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
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
        let expected = cpu.a.wrapping_sub(ram[cpu.hl()]);
        test(&mut cpu, &mut ram, 8, opcode(0x9E));
        assert!(
            cpu.a == expected,
            format!("sbc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        ram[0x1F01] = 0x11;
        cpu.a = 0x12;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_sub(ram[cpu.hl()] + 1);
        test(&mut cpu, &mut ram, 8, opcode(0x9E));
        assert!(
            cpu.a == expected,
            format!("sbc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    }
    test_sbc_a_hl();

    // TODO: test_sbc_a_n
}
