use cpu::tests::*;

#[test]
fn test_cp_a_r() {
    macro_rules! test_cp_a_r {
        ($r:ident, $func:expr) => {{
            let (mut cpu, mut ram) = init(None);
            cpu.a = 0x10;
            cpu.$r = 0x1;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x1F;
            cpu.$r = 0x1;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x0;
            cpu.$r = 0x1;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(cpu.f.h());
            assert!(cpu.f.c());

            cpu.a = 0xF0;
            cpu.$r = 0x11;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(!cpu.f.z());
            assert!(cpu.f.n());
            assert!(cpu.f.h());
            assert!(!cpu.f.c());

            cpu.a = 0x10;
            cpu.$r = 0x10;
            test(&mut cpu, &mut ram, 4, $func);
            assert!(cpu.f.z());
            assert!(cpu.f.n());
            assert!(!cpu.f.h());
            assert!(!cpu.f.c());
        }};
    }
    fn test_cp_a_a() {
        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x4;
        test(&mut cpu, &mut ram, 4, opcode(0xBF));
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        cpu.a = 0x0;
        test(&mut cpu, &mut ram, 4, opcode(0xBF));
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    }

    test_cp_a_a();
    test_cp_a_r!(b, opcode(0xB8));
    test_cp_a_r!(c, opcode(0xB9));
    test_cp_a_r!(d, opcode(0xBA));
    test_cp_a_r!(e, opcode(0xBB));
    test_cp_a_r!(h, opcode(0xBC));
    test_cp_a_r!(l, opcode(0xBD));
    fn test_cp_a_hl() {
        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x10;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        test(&mut cpu, &mut ram, 8, opcode(0xBE));
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x1F;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        test(&mut cpu, &mut ram, 8, opcode(0xBE));
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x1;
        cpu.a = 0x0;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        test(&mut cpu, &mut ram, 8, opcode(0xBE));
        assert!(!cpu.f.z());
        assert!(cpu.f.n());
        assert!(cpu.f.h());
        assert!(cpu.f.c());

        let (mut cpu, mut ram) = init(None);
        ram[0x1F01] = 0x11;
        cpu.a = 0x11;
        cpu.h = 0x1F;
        cpu.l = 0x1;
        test(&mut cpu, &mut ram, 8, opcode(0xBE));
        assert!(cpu.f.z());
        assert!(cpu.f.n());
        assert!(!cpu.f.h());
        assert!(!cpu.f.c());
    }
    test_cp_a_hl();
}
