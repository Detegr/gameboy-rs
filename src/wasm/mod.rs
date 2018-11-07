#[macro_use]
use wasm_bindgen::prelude::*;

mod wasmdisplay;
#[macro_use]
mod wasmlog;

use cpu::Cpu;
use gpu::Gpu;
use mmu::Mmu;

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
    gpu: Gpu,
    mmu: Mmu,
    display: wasmdisplay::WasmDisplay,
}

#[wasm_bindgen]
pub fn reset(emu: &mut Emulator) {
    emu.cpu.reset();
}

#[wasm_bindgen]
pub fn run_until_redraw(emu: &mut Emulator) -> bool {
    while !emu.display.is_dirty() {
        emu.cpu.step(&mut emu.mmu);
        emu.gpu
            .step(&mut emu.display, &mut emu.mmu, emu.cpu.cycles());
    }
    true
}

#[wasm_bindgen]
pub fn display_buffer(emu: &Emulator) -> Vec<u8> {
    emu.display
        .buffer()
        .into_iter()
        .flat_map(|&pixel| vec![pixel, pixel, pixel, 255].into_iter())
        .collect()
}

#[wasm_bindgen]
pub fn load_cartridge_data(emu: &mut Emulator, data: &[u8]) {
    emu.mmu.load_cartridge_data(data);
}

#[wasm_bindgen]
pub fn should_redraw_display(emu: &mut Emulator) -> bool {
    emu.display.is_dirty()
}

fn get_display() -> wasmdisplay::WasmDisplay {
    wasmdisplay::WasmDisplay::new()
}

#[wasm_bindgen]
pub fn get_emulator(rom: &str) -> Emulator {
    info!("Starting {}", rom);

    let cpu = Cpu::new();
    let gpu = Gpu::new();
    let mmu = Mmu::new();
    let display = get_display();

    Emulator {
        cpu,
        gpu,
        mmu,
        display,
    }
}
