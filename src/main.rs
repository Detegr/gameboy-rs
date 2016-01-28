extern crate byteorder;

mod cpu;
mod ram;

use cpu::Cpu;
use ram::Ram;

fn main() {
    let mut cpu = Cpu::new();
    let mut ram = Ram::new();
    execute_next(&mut cpu, &mut ram);
}

pub fn execute_next(cpu: &mut Cpu, ram: &mut Ram) {
    cpu::opcodes::OPCODES[0](cpu, ram);
}
