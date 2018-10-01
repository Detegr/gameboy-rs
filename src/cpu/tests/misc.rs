use cpu::tests::*;

#[test]
fn test_cpl() {
    let (mut cpu, mut ram) = init(None);
    cpu.a = 0x13;
    let expected = 0xEC;

    assert!(!cpu.f.z());
    assert!(!cpu.f.c());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    test(&mut cpu, &mut ram, 4, opcode(0x2F));
    assert!(!cpu.f.z());
    assert!(!cpu.f.c());
    assert!(cpu.f.n());
    assert!(cpu.f.h());
    assert_eq!(cpu.a, expected);
}

#[test]
fn test_scf() {
    let (mut cpu, mut ram) = init(None);
    assert!(!cpu.f.z());
    assert!(!cpu.f.c());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    test(&mut cpu, &mut ram, 4, opcode(0x37));
    assert!(!cpu.f.z());
    assert!(cpu.f.c());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
}
