extern crate byteorder;

mod cpu;
mod ram;

use cpu::Cpu;
use ram::Ram;

fn main() {
    let mut cpu = Cpu::new();
    let mut ram = Ram::new();
    cpu.step(&mut ram);
}
