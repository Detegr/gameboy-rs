use cpu::tests::*;

#[test]
fn test_daa() {
    let (mut cpu, mut ram) = init(None);

    cpu.a = 0x13;
    cpu.b = 0x19;

    cpu.add_a_b(&mut ram);
    assert_eq!(cpu.a, 0x13 + 0x19);

    let expected = 0x32;

    test(&mut cpu, &mut ram, 4, opcode(0x27));
    assert_eq!(cpu.a, expected);

    cpu.a = 0x90;
    cpu.b = 0x20;
    cpu.add_a_b(&mut ram);
    assert_eq!(cpu.a, 0x90 + 0x20);

    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    let expected = 0x10;

    test(&mut cpu, &mut ram, 4, opcode(0x27));

    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());
    assert_eq!(cpu.a, expected);

    // TODO: Test subtraction
}
