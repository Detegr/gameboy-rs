extern crate byteorder;
#[macro_use]
extern crate log;
extern crate simplelog;

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
    info!("Cartridge type: 0x{:X}", mmu.read_u8(0x147));
    loop {
        debug!("{}", cpu);
        cpu.step(&mut mmu);
    }
}
