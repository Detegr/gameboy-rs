use cpu::tests::*;

#[test]
fn test_nop() {
    cycles(8, Cpu::nop);
}
