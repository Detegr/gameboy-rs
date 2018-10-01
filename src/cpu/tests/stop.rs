use cpu;
use cpu::tests::*;

#[test]
fn test_stop() {
    let (mut cpu, mut ram) = init(None);
    cpu.reset();
    assert_eq!(cpu.run_state, cpu::RunState::Running);
    test(&mut cpu, &mut ram, 4, opcode(0x10));
    assert_eq!(cpu.run_state, cpu::RunState::Stopped);

    // TODO: Simulate button press and wake up the processor
}

#[test]
fn test_halt() {
    let (mut cpu, mut ram) = init(None);
    cpu.reset();
    assert_eq!(cpu.run_state, cpu::RunState::Running);
    test(&mut cpu, &mut ram, 4, opcode(0x76));
    assert_eq!(cpu.run_state, cpu::RunState::Halted);

    // TODO: Simulate an interrupt and wake up the processor
}
