use ::cpu::Cpu;
use ::mmu::Mmu;

pub type OpcodeFunction = fn(&mut Cpu, &mut Mmu);
pub static OPCODES: &'static [OpcodeFunction] = &[
    Cpu::nop, Cpu::ld_bc_nn, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::ld_de_nn, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::ld_hl_nn, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::ld_sp_nn, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::ld_a_b, Cpu::ld_a_c, Cpu::ld_a_d, Cpu::ld_a_e, Cpu::ld_a_h, Cpu::ld_a_l, Cpu::ld_a_hl, Cpu::ld_a_a,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
];
