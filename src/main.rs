extern crate byteorder;
#[macro_use]
extern crate log;
extern crate simplelog;

mod cartridge;
mod cpu;
mod mmu;

use cpu::Cpu;
use mmu::Mmu;

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Trace,
        simplelog::Config {
            time: None,
            target: None,
            location: None,
            ..Default::default()
        },
    )
    .unwrap();
    let mut cpu = Cpu::new();
    let mut mmu = Mmu::new();
    mmu.load_cartridge("cpu_instrs/cpu_instrs.gb").unwrap();
    cpu.reset();
    loop {
        debug!("{}", cpu);
        cpu.step(&mut mmu);
    }
}
