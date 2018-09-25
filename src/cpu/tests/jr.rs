use cpu::tests::*;

#[test]
fn test_jr_n() {
    use byteorder::{ByteOrder, LittleEndian};
    let (mut cpu, mut ram) = init(Some(&[0; 0x1000]));
    cpu.reset();

    ram[cpu.pc] = 0x1;

    let expected = cpu.pc + 1;
    test(&mut cpu, &mut ram, 12, opcode(0x18));
    assert_eq!(cpu.pc, expected);

    cpu.pc = 0xFFFF;
    ram[cpu.pc] = 0x1;
    let expected = 1;
    test(&mut cpu, &mut ram, 12, opcode(0x18));
    assert_eq!(cpu.pc, expected);
}
