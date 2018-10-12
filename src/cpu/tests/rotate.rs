use cpu::tests::*;

#[test]
fn test_rra() {
    let (mut cpu, mut ram) = init(None);
    cpu.a = 0x4;
    test(&mut cpu, &mut ram, 4, opcode(0x1F));
    assert_eq!(cpu.a, 0x2);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
    test(&mut cpu, &mut ram, 4, opcode(0x1F));
    assert_eq!(cpu.a, 0x1);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0x5;
    test(&mut cpu, &mut ram, 4, opcode(0x1F));
    assert_eq!(cpu.a, 0x2);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x1F));
    assert_eq!(cpu.a, 0x81);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x1F));
    assert_eq!(cpu.a, 0x40);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x1F));
    assert_eq!(cpu.a, 0xA0);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
}

#[test]
fn test_rla() {
    let (mut cpu, mut ram) = init(None);
    cpu.a = 0x1;
    test(&mut cpu, &mut ram, 4, opcode(0x17));
    assert_eq!(cpu.a, 0x2);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
    test(&mut cpu, &mut ram, 4, opcode(0x17));
    assert_eq!(cpu.a, 0x4);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0xA0;
    test(&mut cpu, &mut ram, 4, opcode(0x17));
    assert_eq!(cpu.a, 0x40);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x17));
    assert_eq!(cpu.a, 0x81);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x17));
    assert_eq!(cpu.a, 0x2);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x17));
    assert_eq!(cpu.a, 0x5);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
}

#[test]
fn test_rrca() {
    let (mut cpu, mut ram) = init(None);
    cpu.a = 0x4;
    test(&mut cpu, &mut ram, 4, opcode(0xF));
    assert_eq!(cpu.a, 0x2);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
    test(&mut cpu, &mut ram, 4, opcode(0xF));
    assert_eq!(cpu.a, 0x1);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0x5;
    test(&mut cpu, &mut ram, 4, opcode(0xF));
    assert_eq!(cpu.a, 0x82);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0xF));
    assert_eq!(cpu.a, 0x41);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0x0;
    test(&mut cpu, &mut ram, 4, opcode(0xF));
    assert_eq!(cpu.a, 0x0);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
}

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
    assert_eq!(cpu.a, 0x41);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(cpu.f.c());

    test(&mut cpu, &mut ram, 4, opcode(0x7));
    assert_eq!(cpu.a, 0x82);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());

    cpu.a = 0x0;
    test(&mut cpu, &mut ram, 4, opcode(0x7));
    assert_eq!(cpu.a, 0x0);
    assert!(!cpu.f.z());
    assert!(!cpu.f.n());
    assert!(!cpu.f.h());
    assert!(!cpu.f.c());
}
