use cpu::tests::*;
use cpu::*;

#[test]
fn test_nop() {
    cycles(8, Cpu::nop);
}
