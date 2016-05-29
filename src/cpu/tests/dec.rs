use cpu::tests::*;

#[test]
fn test_dec_r() {
    macro_rules! test_dec_r(
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.$r = 0x11;
            let expected = cpu.$r.wrapping_sub(0x1);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.$r == expected, format!("dec {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.$r = 0x10;
            let expected = cpu.$r.wrapping_sub(0x1);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.$r == expected, format!("dec {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.$r = 0x1;
            let expected = cpu.$r.wrapping_sub(0x1);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.$r == expected, format!("dec {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(cpu.f.z());
            assert!(!cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            let (mut cpu, mut ram) = init(None);
            cpu.$r = 0x0;
            let expected = cpu.$r.wrapping_sub(0x1);
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.$r == expected, format!("dec {}: Expected 0x{:X}, got 0x{:X}", stringify!($r), expected, cpu.$r));
            assert!(!cpu.f.z());
            assert!(!cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());
        }}
    );
    test_dec_r!(a, opcode(0x3D));
    test_dec_r!(b, opcode(0x5));
    test_dec_r!(c, opcode(0xD));
    test_dec_r!(d, opcode(0x15));
    test_dec_r!(e, opcode(0x1D));
    test_dec_r!(h, opcode(0x25));
    test_dec_r!(l, opcode(0x2D));
    fn test_dec_hl() {
        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x15;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = ram[0x1F01] - 1;
        test(&mut cpu, &mut ram, 8, opcode(0x35));
        assert!(ram[cpu.hl() as usize] == expected, format!("dec (hl): Expected 0x{:X}, got 0x{:X}", expected, ram[cpu.hl() as usize]));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x10;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = ram[0x1F01] - 1;
        test(&mut cpu, &mut ram, 8, opcode(0x35));
        assert!(ram[cpu.hl() as usize] == expected, format!("dec (hl): Expected 0x{:X}, got 0x{:X}", expected, ram[cpu.hl() as usize]));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = ram[0x1F01] - 1;
        test(&mut cpu, &mut ram, 8, opcode(0x35));
        assert!(ram[cpu.hl() as usize] == expected, format!("dec (hl): Expected 0x{:X}, got 0x{:X}", expected, ram[cpu.hl() as usize]));
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x0;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        let expected = ram[0x1F01].wrapping_sub(1);
        test(&mut cpu, &mut ram, 8, opcode(0x35));
        assert!(ram[cpu.hl() as usize] == expected, format!("dec (hl): Expected 0x{:X}, got 0x{:X}", expected, ram[cpu.hl() as usize]));
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());
    }
    test_dec_hl();
}

#[test]
fn test_dec_rr() {
    macro_rules! test_dec_rr(
        ($r1:ident, $r2:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.$r1 = 0x0;
            cpu.$r2 = 0x0;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(cpu.$r1 == 0xFF, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0xFF, cpu.$r1));
            assert!(cpu.$r2 == 0xFF, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0xFF, cpu.$r2));

            cpu.$r1 = 0xFF;
            cpu.$r2 = 0x1;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(cpu.$r1 == 0xFF, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0xFF, cpu.$r1));
            assert!(cpu.$r2 == 0x0, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x0, cpu.$r2));

            cpu.$r1 = 0x1;
            cpu.$r2 = 0x6;
            test(&mut cpu, &mut ram, 8, $func);
            assert!(cpu.$r1 == 0x1, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x1, cpu.$r1));
            assert!(cpu.$r2 == 0x5, format!("dec {}{}: Expected {}, got {}", stringify!($r1), stringify!($r2), 0x5, cpu.$r2));
        }}
    );
    test_dec_rr!(b, c, opcode(0x0B));
    test_dec_rr!(d, e, opcode(0x1B));
    test_dec_rr!(h, l, opcode(0x2B));
}

#[test]
fn test_dec_sp() {
    let (mut cpu, mut ram) = init(None);
    cpu.sp = 1;
    test(&mut cpu, &mut ram, 8, opcode(0x3B));
    assert!(cpu.sp == 0x0);
    test(&mut cpu, &mut ram, 8, opcode(0x3B));
    assert!(cpu.sp == 0xFFFF);
    test(&mut cpu, &mut ram, 8, opcode(0x3B));
    assert!(cpu.sp == 0xFFFE);
}

