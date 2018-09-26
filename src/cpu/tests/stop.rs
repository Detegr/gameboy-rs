use cpu::tests::*;

#[test]
fn test_stop() {
    let (mut cpu, mut ram) = init(None);
    cpu.reset();
    assert!(!cpu.stopped);
    test(&mut cpu, &mut ram, 4, opcode(0x10));
    assert!(cpu.stopped);

    // TODO: Simulate button press and wake up the processor
}
