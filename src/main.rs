extern crate byteorder;
#[macro_use]
extern crate log;
extern crate simplelog;

mod cpu;
mod ram;

use cpu::Cpu;
use ram::Ram;

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
    let mut ram = Ram::new();
    ram.load_cartridge("cpu_instrs/cpu_instrs.gb").unwrap();
    cpu.reset();
    info!("Cartridge type: 0x{:X}", ram[0x147]);
    loop {
        debug!("{}", cpu);
        cpu.step(&mut ram);
    }
}
