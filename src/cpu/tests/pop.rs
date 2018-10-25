use cpu::tests::*;

macro_rules! test_pop_r_r {
    ($r1:ident, $r2:ident, $func:expr) => {
        let (mut cpu, mut mmu) = init(None);
        cpu.reset();
        mmu.write_u8(0xFFFF, 0x11);
        mmu.write_u8(0xFFFE, 0x22);
        cpu.sp = 0xFFFD;

        test(&mut cpu, &mut mmu, 12, $func);

        assert_eq!(cpu.$r1, 0x11);
        assert_eq!(cpu.$r2, 0x22);
        assert_eq!(cpu.sp, 0xFFFF);
    };
}

#[test]
fn test_pop() {
    test_pop_r_r!(b, c, opcode(0xC1));
    test_pop_r_r!(d, e, opcode(0xD1));
    test_pop_r_r!(h, l, opcode(0xE1));

    fn test_pop_a_f() {
        let (mut cpu, mut mmu) = init(None);
        cpu.reset();
        mmu.write_u8(0xFFFF, 0x11);
        mmu.write_u8(0xFFFE, 0xF0);
        cpu.sp = 0xFFFD;

        test(&mut cpu, &mut mmu, 12, opcode(0xF1));

        assert_eq!(cpu.a, 0x11);
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());
        assert_eq!(cpu.sp, 0xFFFF);

        mmu.write_u8(0xFFFF, 0x11);
        mmu.write_u8(0xFFFE, 0x70);
        cpu.sp = 0xFFFD;

        test(&mut cpu, &mut mmu, 12, opcode(0xF1));

        assert_eq!(cpu.a, 0x11);
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());
        assert_eq!(cpu.sp, 0xFFFF);

        mmu.write_u8(0xFFFF, 0x11);
        mmu.write_u8(0xFFFE, 0x30);
        cpu.sp = 0xFFFD;

        test(&mut cpu, &mut mmu, 12, opcode(0xF1));

        assert_eq!(cpu.a, 0x11);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());
        assert_eq!(cpu.sp, 0xFFFF);

        mmu.write_u8(0xFFFF, 0x11);
        mmu.write_u8(0xFFFE, 0x10);
        cpu.sp = 0xFFFD;

        test(&mut cpu, &mut mmu, 12, opcode(0xF1));

        assert_eq!(cpu.a, 0x11);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(cpu.f.c());
        assert_eq!(cpu.sp, 0xFFFF);

        mmu.write_u8(0xFFFF, 0x11);
        mmu.write_u8(0xFFFE, 0x0);
        cpu.sp = 0xFFFD;

        test(&mut cpu, &mut mmu, 12, opcode(0xF1));

        assert_eq!(cpu.a, 0x11);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
        assert_eq!(cpu.sp, 0xFFFF);
    }
    test_pop_a_f();
}
