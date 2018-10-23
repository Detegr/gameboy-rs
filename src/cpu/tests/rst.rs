use cpu::tests::*;

#[test]
fn test_rst() {
    let rst_opcodes = [
        (0xC7, 0x0),
        (0xCF, 0x8),
        (0xD7, 0x10),
        (0xDF, 0x18),
        (0xE7, 0x20),
        (0xEF, 0x28),
        (0xF7, 0x30),
        (0xFF, 0x38),
    ];

    let (mut cpu, mut ram) = init(None);
    for (op, jump_dst) in rst_opcodes.into_iter() {
        cpu.reset();
        cpu.pc = 0x1122;
        test(&mut cpu, &mut ram, 16, opcode(*op));
        assert_eq!(ram[cpu.sp + 1], 0x22);
        assert_eq!(ram[cpu.sp + 2], 0x11);
        assert_eq!(cpu.pc, *jump_dst);
    }
}
