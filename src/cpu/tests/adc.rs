use cpu::tests::*;

#[test]
fn test_adc_a_r() {
    macro_rules! test_adc_a_r {
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut mmu) = init(None);
            cpu.a = 0x10;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
            let carry = 1;
            let expected = cpu.a.wrapping_add(cpu.$r).wrapping_add(carry);
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
            let expected = cpu.a.wrapping_add(cpu.$r).wrapping_add(carry);
            test(&mut cpu, &mut mmu, 4, $func);
            assert!(
                cpu.a == expected,
                format!(
                    "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
    fn test_adc_a_a() {
        let (mut cpu, mut mmu) = init(None);
        cpu.a = 0x4;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut mmu, 4, opcode(0x8F));
        assert!(
            cpu.a == expected,
            format!("adc a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut mmu) = init(None);
        cpu.a = 0x8;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut mmu, 4, opcode(0x8F));
        assert!(
            cpu.a == expected,
            format!("adc a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut mmu) = init(None);
        cpu.a = 0x0;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut mmu, 4, opcode(0x8F));
        assert!(
            cpu.a == expected,
            format!("adc a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut mmu) = init(None);
        cpu.a = 0xF0;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut mmu, 4, opcode(0x8F));
        assert!(
            cpu.a == expected,
            format!("adc a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        cpu.a = 0xF0;
        let expected = cpu.a.wrapping_add(cpu.a) + 1;
        test(&mut cpu, &mut mmu, 4, opcode(0x8F));
        assert!(
            cpu.a == expected,
            format!("adc a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());
    }
    test_adc_a_a();
    test_adc_a_r!(b, opcode(0x88));
    test_adc_a_r!(c, opcode(0x89));
    test_adc_a_r!(d, opcode(0x8A));
    test_adc_a_r!(e, opcode(0x8B));
    test_adc_a_r!(h, opcode(0x8C));
    test_adc_a_r!(l, opcode(0x8D));

    fn test_adc_a_hl() {
        let (mut cpu, mut mmu) = init(None);
        mmu.write_u8(0x1F01, 0x1);
        cpu.a = 0x10;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(mmu.read_u8(cpu.hl()));
        test(&mut cpu, &mut mmu, 8, opcode(0x8E));
        assert!(
            cpu.a == expected,
            format!("adc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut mmu) = init(None);
        mmu.write_u8(0x1F01, 0x1);
        cpu.a = 0x1F;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(mmu.read_u8(cpu.hl()));
        test(&mut cpu, &mut mmu, 8, opcode(0x8E));
        assert!(
            cpu.a == expected,
            format!("adc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut mmu) = init(None);
        mmu.write_u8(0x1F01, 0x1);
        cpu.a = 0xFF;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(mmu.read_u8(cpu.hl()));
        test(&mut cpu, &mut mmu, 8, opcode(0x8E));
        assert!(
            cpu.a == expected,
            format!("adc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        mmu.write_u8(0x1F01, 0x11);
        cpu.a = 0xF0;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(mmu.read_u8(cpu.hl()) + 1);
        test(&mut cpu, &mut mmu, 8, opcode(0x8E));
        assert!(
            cpu.a == expected,
            format!("adc a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a)
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());
    }
    test_adc_a_hl();

    fn test_adc_a_n() {
        let (mut cpu, mut mmu) = init(None);
        cpu.reset();
        let val = 0x1;
        cpu.f.unset_c();
        cpu.a = 0x10;
        mmu.write_u8(cpu.pc, val);
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut mmu, 8, opcode(0xCE));
        assert!(
            cpu.a == expected,
            format!(
                "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
        mmu.write_u8(cpu.pc, val);
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut mmu, 8, opcode(0xCE));
        assert!(
            cpu.a == expected,
            format!(
                "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
        mmu.write_u8(cpu.pc, val);
        let expected = cpu.a.wrapping_add(val);
        test(&mut cpu, &mut mmu, 8, opcode(0xCE));
        assert!(
            cpu.a == expected,
            format!(
                "adc a, {}: Expected 0x{:X}, got 0x{:X}",
                stringify!($r),
                expected,
                cpu.a
            )
        );
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        let val = 0x11;
        cpu.a = 0xF0;
        mmu.write_u8(cpu.pc, val);
        let carry = 1;
        let expected = cpu.a.wrapping_add(val).wrapping_add(carry);
        test(&mut cpu, &mut mmu, 8, opcode(0xCE));
        assert!(
            cpu.a == expected,
            format!(
                "adc a, {}: Expected 0x{:X}, got 0x{:X}",
                stringify!($r),
                expected,
                cpu.a
            )
        );
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        let val = 0x1;
        cpu.a = 0x10;
        mmu.write_u8(cpu.pc, val);
        let expected = cpu.a.wrapping_add(val).wrapping_add(carry);
        test(&mut cpu, &mut mmu, 8, opcode(0xCE));
        assert!(
            cpu.a == expected,
            format!(
                "adc a, {}: Expected 0x{:X}, got 0x{:X}",
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
    test_adc_a_n();
}
