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
            assert!(cpu.a == expected, format!("add a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.a));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.a = 0x10;
            cpu.$r = 0x10;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.a == expected, format!("add a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.a));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.a = 0xFF;
            cpu.$r = 0x1;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.a == expected, format!("add a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.a));
            assert!(cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.a = 0xF0;
            cpu.$r = 0x11;
            let expected = cpu.a.wrapping_add(cpu.$r);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.a == expected, format!("add a, {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.a));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());
        }}
    }
    fn test_add_a_a() {
        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x4;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(cpu.a == expected, format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x8;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(cpu.a == expected, format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x0;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(cpu.a == expected, format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0xF0;
        let expected = cpu.a.wrapping_add(cpu.a);
        test(&mut cpu, &mut ram, 4, opcode(0x87));
        assert!(cpu.a == expected, format!("add a, a: Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
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
        let expected = cpu.a.wrapping_add(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(cpu.a == expected, format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x10;
        cpu.a = 0x10;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(cpu.a == expected, format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0xFF;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(cpu.a == expected, format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x11;
        cpu.a = 0xF0;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = cpu.a.wrapping_add(ram[cpu.hl() as usize]);
        test(&mut cpu, &mut ram, 8, opcode(0x86));
        assert!(cpu.a == expected, format!("add a, (hl): Expected 0x{:X}, got 0x{:X}", expected, cpu.a));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());
    }
    test_add_a_hl();
}

