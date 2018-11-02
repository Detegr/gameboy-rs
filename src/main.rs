extern crate byteorder;
#[macro_use]
extern crate log;
extern crate simplelog;

#[cfg(feature = "glfb")]
mod gl_display;

mod cartridge;
mod cpu;
mod display;
mod gpu;
mod mmu;

use cpu::Cpu;
use gpu::Gpu;
use mmu::Mmu;

#[cfg(not(feature = "glfb"))]
fn get_display() -> display::DebugDisplay {
    display::DebugDisplay
}

#[cfg(feature = "glfb")]
fn get_display() -> gl_display::GlDisplay {
    gl_display::GlDisplay::new()
}

fn main() {
    let mut args = std::env::args();
    let filename = args.nth(1);
    simplelog::SimpleLogger::init(
        simplelog::LevelFilter::Info,
        simplelog::Config {
            time: None,
            target: None,
            location: None,
            ..Default::default()
        },
    )
    .unwrap();

    let mut cpu = Cpu::new();
    let mut gpu = Gpu::new();
    let mut mmu = Mmu::new();
    let mut display = get_display();

    //mmu.load_cartridge("cpu_instrs/cpu_instrs.gb").unwrap();
    if let Some(filename) = filename {
        mmu.load_cartridge(&filename).unwrap();
    } else {
        mmu.load_cartridge("cpu_instrs/cpu_instrs.gb").unwrap();
    }
    cpu.reset();
    loop {
        if cpu.cycles > 8488916 && cpu.cycles < 8491540 {
            //cpu.debug = true;
            //info!("{}", cpu);
        } else {
            cpu.debug = false;
        }
        //debug!("{:?}", gpu);
        cpu.step(&mut mmu);
        gpu.step(&mut display, &mut mmu, cpu.cycles());
    }
}
