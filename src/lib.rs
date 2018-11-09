#[macro_use]
extern crate log;

pub mod cartridge;
pub mod cpu;
pub mod display;
pub mod gpu;
pub mod mmu;

#[cfg(target_os = "unknown")]
extern crate wasm_bindgen;

#[cfg(target_os = "unknown")]
extern crate wasm_logger;

#[cfg(target_os = "unknown")]
mod wasm;
