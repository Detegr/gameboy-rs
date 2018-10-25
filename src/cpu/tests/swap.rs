use cpu::tests::*;

macro_rules! test_swap_r {
    ($r:ident, $func: expr) => {
        let (mut cpu, mut mmu) = init(None);

        cpu.$r = 0xF0;
        test(&mut cpu, &mut mmu, 8, $func);
        assert_eq!(cpu.$r, 0xF);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        cpu.$r = 0xF;
        test(&mut cpu, &mut mmu, 8, $func);
        assert_eq!(cpu.$r, 0xF0);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        cpu.$r = 0x12;
        test(&mut cpu, &mut mmu, 8, $func);
        assert_eq!(cpu.$r, 0x21);
        assert!(!cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        cpu.$r = 0x0;
        test(&mut cpu, &mut mmu, 8, $func);
        assert_eq!(cpu.$r, 0x0);
        assert!(cpu.f.z());
        assert!(!cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    };
}

#[test]
fn test_swap() {
    test_swap_r!(b, opcode(0x100 + 0x30));
    test_swap_r!(c, opcode(0x100 + 0x31));
    test_swap_r!(d, opcode(0x100 + 0x32));
    test_swap_r!(e, opcode(0x100 + 0x33));
    test_swap_r!(h, opcode(0x100 + 0x34));
    test_swap_r!(l, opcode(0x100 + 0x35));
    test_swap_r!(a, opcode(0x100 + 0x37));
}
