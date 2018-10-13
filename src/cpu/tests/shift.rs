use cpu::tests::*;

macro_rules! test_sla_r {
    ($r:ident, $func: expr) => {
        let (mut cpu, mut ram) = init(None);
        cpu.f.set_c();
        cpu.f.set_z();
        cpu.f.set_n();
        cpu.f.set_h();

        cpu.$r = 0xA0;
        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x40);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x80);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x0);
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x0);
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    };
}

#[test]
fn test_sla_r() {
    test_sla_r!(b, opcode(0x100 + 0x20));
    test_sla_r!(c, opcode(0x100 + 0x21));
    test_sla_r!(d, opcode(0x100 + 0x22));
    test_sla_r!(e, opcode(0x100 + 0x23));
    test_sla_r!(h, opcode(0x100 + 0x24));
    test_sla_r!(l, opcode(0x100 + 0x25));
    test_sla_r!(a, opcode(0x100 + 0x27));
}

macro_rules! test_sra_r {
    ($r:ident, $func: expr) => {
        let (mut cpu, mut ram) = init(None);
        cpu.f.set_c();
        cpu.f.set_z();
        cpu.f.set_n();
        cpu.f.set_h();

        cpu.$r = 0x85;
        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0xC2);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0xE1);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0xF0);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        cpu.$r = 0x1;
        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x0);
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x0);
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    };
}

#[test]
fn test_sra_r() {
    test_sra_r!(b, opcode(0x100 + 0x28));
    test_sra_r!(c, opcode(0x100 + 0x29));
    test_sra_r!(d, opcode(0x100 + 0x2A));
    test_sra_r!(e, opcode(0x100 + 0x2B));
    test_sra_r!(h, opcode(0x100 + 0x2C));
    test_sra_r!(l, opcode(0x100 + 0x2D));
    test_sra_r!(a, opcode(0x100 + 0x2F));
}

macro_rules! test_srl_r {
    ($r:ident, $func: expr) => {
        let (mut cpu, mut ram) = init(None);
        cpu.f.set_c();
        cpu.f.set_z();
        cpu.f.set_n();
        cpu.f.set_h();

        cpu.$r = 0x85;
        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x42);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x21);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x10);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        cpu.$r = 0x1;
        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x0);
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());

        test(&mut cpu, &mut ram, 8, $func);
        assert_eq!(cpu.$r, 0x0);
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    };
}

#[test]
fn test_srl_r() {
    test_srl_r!(b, opcode(0x100 + 0x38));
    test_srl_r!(c, opcode(0x100 + 0x39));
    test_srl_r!(d, opcode(0x100 + 0x3A));
    test_srl_r!(e, opcode(0x100 + 0x3B));
    test_srl_r!(h, opcode(0x100 + 0x3C));
    test_srl_r!(l, opcode(0x100 + 0x3D));
    test_srl_r!(a, opcode(0x100 + 0x3F));
}
