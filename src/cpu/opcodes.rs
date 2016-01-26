use ::cpu::Cpu;
use ::mmu::Mmu;

pub type OpcodeFunction = fn(&mut Cpu, &mut Mmu);
pub static OPCODES: &'static [OpcodeFunction] = &[
    Cpu::nop,
    Cpu::ld_bc_nn,
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::ld_de_nn,
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::ld_hl_nn,
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::nop, // TODO
    Cpu::ld_sp_nn,
];

