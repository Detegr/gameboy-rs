extern crate byteorder;

mod cpu;
mod mmu;

use cpu::Cpu;
use mmu::Mmu;

fn main() {
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
    execute_next(&mut cpu, &mut mmu);
}

pub fn execute_next(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu::opcodes::OPCODES[0](cpu, mmu);
}
