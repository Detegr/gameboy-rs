extern crate gameboy;
extern crate simplelog;

#[cfg(feature = "glfb")]
mod gl_display;

use gameboy::*;

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
    let boot_rom = args.nth(0);
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
    let mut mmu = Mmu::new(&boot_rom);
    let mut display = get_display();

    if let Some(filename) = filename {
        mmu.load_cartridge(&filename).unwrap();
    } else {
        mmu.load_cartridge("cpu_instrs/cpu_instrs.gb").unwrap();
    }

    if boot_rom.is_none() {
        cpu.reset();
    }

    loop {
        //info!("{}", cpu);
        cpu.step(&mut mmu);
        gpu.step(&mut display, &mut mmu, cpu.cycles());
    }
}
