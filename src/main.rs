extern crate byteorder;

mod cpu;
mod ram;

use cpu::Cpu;
use ram::Ram;

fn main() {
    let mut cpu = Cpu::new();
    let mut ram = Ram::new();
    ram.load_cartridge("cpu_instrs/cpu_instrs.gb").unwrap();
    cpu.reset();
    loop {
        println!("{:?}", cpu);
        cpu.step(&mut ram);
    }
}
