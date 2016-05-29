use ::cpu::Cpu;
use ::ram::Ram;

pub type OpcodeFunction = fn(&mut Cpu, &mut Ram);
pub static OPCODES: &'static [OpcodeFunction] = &[
    Cpu::nop, Cpu::ld_bc_nn, Cpu::ld_bc_a, Cpu::inc_combined_bc, Cpu::nyi, Cpu::dec_b, Cpu::ld_b_n, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::ld_a_bc, Cpu::dec_combined_bc, Cpu::nyi, Cpu::dec_c, Cpu::ld_c_n, Cpu::nyi,
    Cpu::nyi, Cpu::ld_de_nn, Cpu::ld_de_a, Cpu::inc_combined_de, Cpu::nyi, Cpu::dec_d, Cpu::ld_d_n, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::ld_a_de, Cpu::dec_combined_de, Cpu::nyi, Cpu::dec_e, Cpu::ld_e_n, Cpu::nyi,
    Cpu::nyi, Cpu::ld_hl_nn, Cpu::nyi, Cpu::inc_combined_hl, Cpu::nyi, Cpu::dec_h, Cpu::ld_h_n, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::dec_combined_hl, Cpu::nyi, Cpu::dec_l, Cpu::ld_l_n, Cpu::nyi,
    Cpu::nyi, Cpu::ld_sp_nn, Cpu::nyi, Cpu::inc_combined_sp, Cpu::nyi, Cpu::dec_hl, Cpu::ld_hl_n, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::dec_combined_sp, Cpu::nyi, Cpu::dec_a, Cpu::ld_a_n, Cpu::nyi,
    Cpu::ld_b_b, Cpu::ld_b_c, Cpu::ld_b_d, Cpu::ld_b_e, Cpu::ld_b_h, Cpu::ld_b_l, Cpu::ld_b_hl, Cpu::ld_b_a, Cpu::ld_c_b, Cpu::ld_c_c, Cpu::ld_c_d, Cpu::ld_c_e, Cpu::ld_c_h, Cpu::ld_c_l, Cpu::ld_c_hl, Cpu::ld_c_a,
    Cpu::ld_d_b, Cpu::ld_d_c, Cpu::ld_d_d, Cpu::ld_d_e, Cpu::ld_d_h, Cpu::ld_d_l, Cpu::ld_d_hl, Cpu::ld_d_a, Cpu::ld_e_b, Cpu::ld_e_c, Cpu::ld_e_d, Cpu::ld_e_e, Cpu::ld_e_h, Cpu::ld_e_l, Cpu::ld_e_hl, Cpu::ld_e_a,
    Cpu::ld_h_b, Cpu::ld_h_c, Cpu::ld_h_d, Cpu::ld_h_e, Cpu::ld_h_h, Cpu::ld_h_l, Cpu::ld_h_hl, Cpu::ld_h_a, Cpu::ld_l_b, Cpu::ld_l_c, Cpu::ld_l_d, Cpu::ld_l_e, Cpu::ld_l_h, Cpu::ld_l_l, Cpu::ld_l_hl, Cpu::ld_l_a,
    Cpu::ld_hl_b, Cpu::ld_hl_c, Cpu::ld_hl_d, Cpu::ld_hl_e, Cpu::ld_hl_h, Cpu::ld_hl_l, Cpu::nyi, Cpu::ld_hl_a, Cpu::ld_a_b, Cpu::ld_a_c, Cpu::ld_a_d, Cpu::ld_a_e, Cpu::ld_a_h, Cpu::ld_a_l, Cpu::ld_a_hl, Cpu::ld_a_a,
    Cpu::add_a_b, Cpu::add_a_c, Cpu::add_a_d, Cpu::add_a_e, Cpu::add_a_h, Cpu::add_a_l, Cpu::add_a_hl, Cpu::add_a_a, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::ld_addr_c_a, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::ld_nn_a, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
    Cpu::nyi, Cpu::nyi, Cpu::ld_a_addr_c, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::ld_a_nn, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi, Cpu::nyi,
];
