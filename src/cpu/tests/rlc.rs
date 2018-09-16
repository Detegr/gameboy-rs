use cpu::tests::*;

#[test]
fn test_rlca() {
    let (mut cpu, mut ram) = init(None);
    cpu.a = 0x1;
    test(&mut cpu, &mut ram, 4, opcode(0x7));
    assert_eq!(cpu.a, 0x2);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
    test(&mut cpu, &mut ram, 4, opcode(0x7));
    assert_eq!(cpu.a, 0x4);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0xA0;
    test(&mut cpu, &mut ram, 4, opcode(0x7));
    assert_eq!(cpu.a, 0x40);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x7));
    assert_eq!(cpu.a, 0x80);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
}
