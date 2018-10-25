use cpu::Cpu;
use mmu::Mmu;

pub type OpcodeFunction = fn(&mut Cpu, &mut Mmu);
pub static OPCODES: [OpcodeFunction; 512] = [
    Cpu::nop,
    Cpu::ld_bc_nn,
    Cpu::ld_bc_a,
    Cpu::inc_bc,
    Cpu::inc_b,
    Cpu::dec_b,
    Cpu::ld_b_n,
    Cpu::rlca,
    Cpu::ld_deref_a16_sp,
    Cpu::add_hl_bc,
    Cpu::ld_a_deref_bc,
    Cpu::dec_bc,
    Cpu::inc_c,
    Cpu::dec_c,
    Cpu::ld_c_n,
    Cpu::rrca,
    Cpu::stop,
    Cpu::ld_de_nn,
    Cpu::ld_de_a,
    Cpu::inc_de,
    Cpu::inc_d,
    Cpu::dec_d,
    Cpu::ld_d_n,
    Cpu::rla,
    Cpu::jr_n,
    Cpu::add_hl_de,
    Cpu::ld_a_deref_de,
    Cpu::dec_de,
    Cpu::inc_e,
    Cpu::dec_e,
    Cpu::ld_e_n,
    Cpu::rra,
    Cpu::jr_nz_n,
    Cpu::ld_hl_nn,
    Cpu::ld_hli_a,
    Cpu::inc_hl,
    Cpu::inc_h,
    Cpu::dec_h,
    Cpu::ld_h_n,
    Cpu::daa,
    Cpu::jr_z_n,
    Cpu::add_hl_hl,
    Cpu::ld_a_hli,
    Cpu::dec_hl,
    Cpu::inc_l,
    Cpu::dec_l,
    Cpu::ld_l_n,
    Cpu::cpl,
    Cpu::jr_nc_n,
    Cpu::ld_sp_nn,
    Cpu::ld_hld_a,
    Cpu::inc_sp,
    Cpu::inc_deref_hl,
    Cpu::dec_deref_hl,
    Cpu::ld_deref_hl_n,
    Cpu::scf,
    Cpu::jr_c_n,
    Cpu::add_hl_sp,
    Cpu::ld_a_hld,
    Cpu::dec_sp,
    Cpu::inc_a,
    Cpu::dec_a,
    Cpu::ld_a_n,
    Cpu::ccf,
    Cpu::ld_b_b,
    Cpu::ld_b_c,
    Cpu::ld_b_d,
    Cpu::ld_b_e,
    Cpu::ld_b_h,
    Cpu::ld_b_l,
    Cpu::ld_b_deref_hl,
    Cpu::ld_b_a,
    Cpu::ld_c_b,
    Cpu::ld_c_c,
    Cpu::ld_c_d,
    Cpu::ld_c_e,
    Cpu::ld_c_h,
    Cpu::ld_c_l,
    Cpu::ld_c_deref_hl,
    Cpu::ld_c_a,
    Cpu::ld_d_b,
    Cpu::ld_d_c,
    Cpu::ld_d_d,
    Cpu::ld_d_e,
    Cpu::ld_d_h,
    Cpu::ld_d_l,
    Cpu::ld_d_deref_hl,
    Cpu::ld_d_a,
    Cpu::ld_e_b,
    Cpu::ld_e_c,
    Cpu::ld_e_d,
    Cpu::ld_e_e,
    Cpu::ld_e_h,
    Cpu::ld_e_l,
    Cpu::ld_e_deref_hl,
    Cpu::ld_e_a,
    Cpu::ld_h_b,
    Cpu::ld_h_c,
    Cpu::ld_h_d,
    Cpu::ld_h_e,
    Cpu::ld_h_h,
    Cpu::ld_h_l,
    Cpu::ld_h_deref_hl,
    Cpu::ld_h_a,
    Cpu::ld_l_b,
    Cpu::ld_l_c,
    Cpu::ld_l_d,
    Cpu::ld_l_e,
    Cpu::ld_l_h,
    Cpu::ld_l_l,
    Cpu::ld_l_deref_hl,
    Cpu::ld_l_a,
    Cpu::ld_hl_b,
    Cpu::ld_hl_c,
    Cpu::ld_hl_d,
    Cpu::ld_hl_e,
    Cpu::ld_hl_h,
    Cpu::ld_hl_l,
    Cpu::halt,
    Cpu::ld_hl_a,
    Cpu::ld_a_b,
    Cpu::ld_a_c,
    Cpu::ld_a_d,
    Cpu::ld_a_e,
    Cpu::ld_a_h,
    Cpu::ld_a_l,
    Cpu::ld_a_deref_hl,
    Cpu::ld_a_a,
    Cpu::add_a_b,
    Cpu::add_a_c,
    Cpu::add_a_d,
    Cpu::add_a_e,
    Cpu::add_a_h,
    Cpu::add_a_l,
    Cpu::add_a_deref_hl,
    Cpu::add_a_a,
    Cpu::adc_a_b,
    Cpu::adc_a_c,
    Cpu::adc_a_d,
    Cpu::adc_a_e,
    Cpu::adc_a_h,
    Cpu::adc_a_l,
    Cpu::adc_a_deref_hl,
    Cpu::adc_a_a,
    Cpu::sub_a_b,
    Cpu::sub_a_c,
    Cpu::sub_a_d,
    Cpu::sub_a_e,
    Cpu::sub_a_h,
    Cpu::sub_a_l,
    Cpu::sub_a_deref_hl,
    Cpu::sub_a_a,
    Cpu::sbc_a_b,
    Cpu::sbc_a_c,
    Cpu::sbc_a_d,
    Cpu::sbc_a_e,
    Cpu::sbc_a_h,
    Cpu::sbc_a_l,
    Cpu::sbc_a_deref_hl,
    Cpu::sbc_a_a,
    Cpu::and_a_b,
    Cpu::and_a_c,
    Cpu::and_a_d,
    Cpu::and_a_e,
    Cpu::and_a_h,
    Cpu::and_a_l,
    Cpu::and_a_deref_hl,
    Cpu::and_a_a,
    Cpu::xor_a_b,
    Cpu::xor_a_c,
    Cpu::xor_a_d,
    Cpu::xor_a_e,
    Cpu::xor_a_h,
    Cpu::xor_a_l,
    Cpu::xor_a_deref_hl,
    Cpu::xor_a_a,
    Cpu::or_a_b,
    Cpu::or_a_c,
    Cpu::or_a_d,
    Cpu::or_a_e,
    Cpu::or_a_h,
    Cpu::or_a_l,
    Cpu::or_a_deref_hl,
    Cpu::or_a_a,
    Cpu::cp_a_b,
    Cpu::cp_a_c,
    Cpu::cp_a_d,
    Cpu::cp_a_e,
    Cpu::cp_a_h,
    Cpu::cp_a_l,
    Cpu::cp_a_deref_hl,
    Cpu::cp_a_a,
    Cpu::ret_nz,
    Cpu::pop_bc,
    Cpu::jp_nz,
    Cpu::jp,
    Cpu::call_nz,
    Cpu::push_bc,
    Cpu::add_a_n,
    Cpu::rst_00h,
    Cpu::ret_z,
    Cpu::ret,
    Cpu::jp_z,
    Cpu::na, // Prefix CB, implemented in Cpu::step()
    Cpu::call_z,
    Cpu::call,
    Cpu::adc_a_n,
    Cpu::rst_08h,
    Cpu::ret_nc,
    Cpu::pop_de,
    Cpu::jp_nc,
    Cpu::na,
    Cpu::call_nc,
    Cpu::push_de,
    Cpu::sub_a_n,
    Cpu::rst_10h,
    Cpu::ret_c,
    Cpu::reti,
    Cpu::jp_c,
    Cpu::na,
    Cpu::call_c,
    Cpu::na,
    Cpu::sbc_a_n,
    Cpu::rst_18h,
    Cpu::ldh_deref_n_a,
    Cpu::pop_hl,
    Cpu::ld_addr_c_a,
    Cpu::na,
    Cpu::na,
    Cpu::push_hl,
    Cpu::and_a_n,
    Cpu::rst_20h,
    Cpu::add_sp_n,
    Cpu::jp_hl,
    Cpu::ld_nn_a,
    Cpu::na,
    Cpu::na,
    Cpu::na,
    Cpu::xor_a_n,
    Cpu::rst_28h,
    Cpu::ldh_a_deref_n,
    Cpu::pop_af,
    Cpu::ld_a_addr_c,
    Cpu::di,
    Cpu::na,
    Cpu::push_af,
    Cpu::or_a_n,
    Cpu::rst_30h,
    Cpu::ld_hl_sp_plus_n,
    Cpu::ld_sp_hl,
    Cpu::ld_a_nn,
    Cpu::ei,
    Cpu::na,
    Cpu::na,
    Cpu::cp_a_n,
    Cpu::rst_38h,
    Cpu::rlc_b,
    Cpu::rlc_c,
    Cpu::rlc_d,
    Cpu::rlc_e,
    Cpu::rlc_h,
    Cpu::rlc_l,
    Cpu::rlc_deref_hl,
    Cpu::rlc_a,
    Cpu::rrc_b,
    Cpu::rrc_c,
    Cpu::rrc_d,
    Cpu::rrc_e,
    Cpu::rrc_h,
    Cpu::rrc_l,
    Cpu::rrc_deref_hl,
    Cpu::rrc_a,
    Cpu::rl_b,
    Cpu::rl_c,
    Cpu::rl_d,
    Cpu::rl_e,
    Cpu::rl_h,
    Cpu::rl_l,
    Cpu::rl_deref_hl,
    Cpu::rl_a,
    Cpu::rr_b,
    Cpu::rr_c,
    Cpu::rr_d,
    Cpu::rr_e,
    Cpu::rr_h,
    Cpu::rr_l,
    Cpu::rr_deref_hl,
    Cpu::rr_a,
    Cpu::sla_b,
    Cpu::sla_c,
    Cpu::sla_d,
    Cpu::sla_e,
    Cpu::sla_h,
    Cpu::sla_l,
    Cpu::sla_deref_hl,
    Cpu::sla_a,
    Cpu::sra_b,
    Cpu::sra_c,
    Cpu::sra_d,
    Cpu::sra_e,
    Cpu::sra_h,
    Cpu::sra_l,
    Cpu::sra_deref_hl,
    Cpu::sra_a,
    Cpu::swap_b,
    Cpu::swap_c,
    Cpu::swap_d,
    Cpu::swap_e,
    Cpu::swap_h,
    Cpu::swap_l,
    Cpu::swap_deref_hl,
    Cpu::swap_a,
    Cpu::srl_b,
    Cpu::srl_c,
    Cpu::srl_d,
    Cpu::srl_e,
    Cpu::srl_h,
    Cpu::srl_l,
    Cpu::srl_deref_hl,
    Cpu::srl_a,
    Cpu::bit0_b,
    Cpu::bit0_c,
    Cpu::bit0_d,
    Cpu::bit0_e,
    Cpu::bit0_h,
    Cpu::bit0_l,
    Cpu::bit0_deref_hl,
    Cpu::bit0_a,
    Cpu::bit1_b,
    Cpu::bit1_c,
    Cpu::bit1_d,
    Cpu::bit1_e,
    Cpu::bit1_h,
    Cpu::bit1_l,
    Cpu::bit1_deref_hl,
    Cpu::bit1_a,
    Cpu::bit2_b,
    Cpu::bit2_c,
    Cpu::bit2_d,
    Cpu::bit2_e,
    Cpu::bit2_h,
    Cpu::bit2_l,
    Cpu::bit2_deref_hl,
    Cpu::bit2_a,
    Cpu::bit3_b,
    Cpu::bit3_c,
    Cpu::bit3_d,
    Cpu::bit3_e,
    Cpu::bit3_h,
    Cpu::bit3_l,
    Cpu::bit3_deref_hl,
    Cpu::bit3_a,
    Cpu::bit4_b,
    Cpu::bit4_c,
    Cpu::bit4_d,
    Cpu::bit4_e,
    Cpu::bit4_h,
    Cpu::bit4_l,
    Cpu::bit4_deref_hl,
    Cpu::bit4_a,
    Cpu::bit5_b,
    Cpu::bit5_c,
    Cpu::bit5_d,
    Cpu::bit5_e,
    Cpu::bit5_h,
    Cpu::bit5_l,
    Cpu::bit5_deref_hl,
    Cpu::bit5_a,
    Cpu::bit6_b,
    Cpu::bit6_c,
    Cpu::bit6_d,
    Cpu::bit6_e,
    Cpu::bit6_h,
    Cpu::bit6_l,
    Cpu::bit6_deref_hl,
    Cpu::bit6_a,
    Cpu::bit7_b,
    Cpu::bit7_c,
    Cpu::bit7_d,
    Cpu::bit7_e,
    Cpu::bit7_h,
    Cpu::bit7_l,
    Cpu::bit7_deref_hl,
    Cpu::bit7_a,
    Cpu::res0_b,
    Cpu::res0_c,
    Cpu::res0_d,
    Cpu::res0_e,
    Cpu::res0_h,
    Cpu::res0_l,
    Cpu::res0_deref_hl,
    Cpu::res0_a,
    Cpu::res1_b,
    Cpu::res1_c,
    Cpu::res1_d,
    Cpu::res1_e,
    Cpu::res1_h,
    Cpu::res1_l,
    Cpu::res1_deref_hl,
    Cpu::res1_a,
    Cpu::res2_b,
    Cpu::res2_c,
    Cpu::res2_d,
    Cpu::res2_e,
    Cpu::res2_h,
    Cpu::res2_l,
    Cpu::res2_deref_hl,
    Cpu::res2_a,
    Cpu::res3_b,
    Cpu::res3_c,
    Cpu::res3_d,
    Cpu::res3_e,
    Cpu::res3_h,
    Cpu::res3_l,
    Cpu::res3_deref_hl,
    Cpu::res3_a,
    Cpu::res4_b,
    Cpu::res4_c,
    Cpu::res4_d,
    Cpu::res4_e,
    Cpu::res4_h,
    Cpu::res4_l,
    Cpu::res4_deref_hl,
    Cpu::res4_a,
    Cpu::res5_b,
    Cpu::res5_c,
    Cpu::res5_d,
    Cpu::res5_e,
    Cpu::res5_h,
    Cpu::res5_l,
    Cpu::res5_deref_hl,
    Cpu::res5_a,
    Cpu::res6_b,
    Cpu::res6_c,
    Cpu::res6_d,
    Cpu::res6_e,
    Cpu::res6_h,
    Cpu::res6_l,
    Cpu::res6_deref_hl,
    Cpu::res6_a,
    Cpu::res7_b,
    Cpu::res7_c,
    Cpu::res7_d,
    Cpu::res7_e,
    Cpu::res7_h,
    Cpu::res7_l,
    Cpu::res7_deref_hl,
    Cpu::res7_a,
    Cpu::set0_b,
    Cpu::set0_c,
    Cpu::set0_d,
    Cpu::set0_e,
    Cpu::set0_h,
    Cpu::set0_l,
    Cpu::set0_deref_hl,
    Cpu::set0_a,
    Cpu::set1_b,
    Cpu::set1_c,
    Cpu::set1_d,
    Cpu::set1_e,
    Cpu::set1_h,
    Cpu::set1_l,
    Cpu::set1_deref_hl,
    Cpu::set1_a,
    Cpu::set2_b,
    Cpu::set2_c,
    Cpu::set2_d,
    Cpu::set2_e,
    Cpu::set2_h,
    Cpu::set2_l,
    Cpu::set2_deref_hl,
    Cpu::set2_a,
    Cpu::set3_b,
    Cpu::set3_c,
    Cpu::set3_d,
    Cpu::set3_e,
    Cpu::set3_h,
    Cpu::set3_l,
    Cpu::set3_deref_hl,
    Cpu::set3_a,
    Cpu::set4_b,
    Cpu::set4_c,
    Cpu::set4_d,
    Cpu::set4_e,
    Cpu::set4_h,
    Cpu::set4_l,
    Cpu::set4_deref_hl,
    Cpu::set4_a,
    Cpu::set5_b,
    Cpu::set5_c,
    Cpu::set5_d,
    Cpu::set5_e,
    Cpu::set5_h,
    Cpu::set5_l,
    Cpu::set5_deref_hl,
    Cpu::set5_a,
    Cpu::set6_b,
    Cpu::set6_c,
    Cpu::set6_d,
    Cpu::set6_e,
    Cpu::set6_h,
    Cpu::set6_l,
    Cpu::set6_deref_hl,
    Cpu::set6_a,
    Cpu::set7_b,
    Cpu::set7_c,
    Cpu::set7_d,
    Cpu::set7_e,
    Cpu::set7_h,
    Cpu::set7_l,
    Cpu::set7_deref_hl,
    Cpu::set7_a,
];

pub static MNEMONICS: [&'static str; 512] = [
    "NOP",
    "LD BC, NN",
    "LD (BC), A",
    "INC BC",
    "INC B",
    "DEC B",
    "LD B, N",
    "RLCA",
    "LD (NN), SP",
    "ADD HL, BC",
    "LD A, (BC)",
    "DEC BC",
    "INC C",
    "DEC C",
    "LD C, N",
    "RRCA",
    "STOP",
    "LD DE, NN",
    "LD (DE), A",
    "INC DE",
    "INC D",
    "DEC D",
    "LD D, N",
    "RLA",
    "JR SN",
    "ADD HL, DE",
    "LD A, (DE)",
    "DEC DE",
    "INC E",
    "DEC E",
    "LD E, N",
    "RRA",
    "JR NZ, SN",
    "LD HL, NN",
    "LDI (HL), A",
    "INC HL",
    "INC H",
    "DEC H",
    "LD H, N",
    "DAA",
    "JR Z, SN",
    "ADD HL, HL",
    "LDI A, (HL)",
    "DEC HL",
    "INC L",
    "DEC L",
    "LD L, N",
    "CPL",
    "JR NC, SN",
    "LD SP, NN",
    "LDD (HL), A",
    "INC SP",
    "INC (HL)",
    "DEC (HL)",
    "LD (HL), N",
    "SCF",
    "JR C, SN",
    "ADD HL, SP",
    "LDD A, (HL)",
    "DEC SP",
    "INC A",
    "DEC A",
    "LD A, N",
    "CCF",
    "LD B, B",
    "LD B, C",
    "LD B, D",
    "LD B, E",
    "LD B, H",
    "LD B, L",
    "LD B, (HL)",
    "LD B, A",
    "LD C, B",
    "LD C, C",
    "LD C, D",
    "LD C, E",
    "LD C, H",
    "LD C, L",
    "LD C, (HL)",
    "LD C, A",
    "LD D, B",
    "LD D, C",
    "LD D, D",
    "LD D, E",
    "LD D, H",
    "LD D, L",
    "LD D, (HL)",
    "LD D, A",
    "LD E, B",
    "LD E, C",
    "LD E, D",
    "LD E, E",
    "LD E, H",
    "LD E, L",
    "LD E, (HL)",
    "LD E, A",
    "LD H, B",
    "LD H, C",
    "LD H, D",
    "LD H, E",
    "LD H, H",
    "LD H, L",
    "LD H, (HL)",
    "LD H, A",
    "LD L, B",
    "LD L, C",
    "LD L, D",
    "LD L, E",
    "LD L, H",
    "LD L, L",
    "LD L, (HL)",
    "LD L, A",
    "LD (HL), B",
    "LD (HL), C",
    "LD (HL), D",
    "LD (HL), E",
    "LD (HL), H",
    "LD (HL), L",
    "HALT",
    "LD (HL), A",
    "LD A, B",
    "LD A, C",
    "LD A, D",
    "LD A, E",
    "LD A, H",
    "LD A, L",
    "LD A, (HL)",
    "LD A, A",
    "ADD A, B",
    "ADD A, C",
    "ADD A, D",
    "ADD A, E",
    "ADD A, H",
    "ADD A, L",
    "ADD A, (HL)",
    "ADD A, A",
    "ADC A, B",
    "ADC A, C",
    "ADC A, D",
    "ADC A, E",
    "ADC A, H",
    "ADC A, L",
    "ADC A, (HL)",
    "ADC A, A",
    "SUB A, B",
    "SUB A, C",
    "SUB A, D",
    "SUB A, E",
    "SUB A, H",
    "SUB A, L",
    "SUB A, (HL)",
    "SUB A, A",
    "SBC A, B",
    "SBC A, C",
    "SBC A, D",
    "SBC A, E",
    "SBC A, H",
    "SBC A, L",
    "SBC A, (HL)",
    "SBC A, A",
    "AND A, B",
    "AND A, C",
    "AND A, D",
    "AND A, E",
    "AND A, H",
    "AND A, L",
    "AND A, (HL)",
    "AND A, A",
    "XOR A, B",
    "XOR A, C",
    "XOR A, D",
    "XOR A, E",
    "XOR A, H",
    "XOR A, L",
    "XOR A, (HL)",
    "XOR A, A",
    "OR A, B",
    "OR A, C",
    "OR A, D",
    "OR A, E",
    "OR A, H",
    "OR A, L",
    "OR A, (HL)",
    "OR A, A",
    "CP A, B",
    "CP A, C",
    "CP A, D",
    "CP A, E",
    "CP A, H",
    "CP A, L",
    "CP A, (HL)",
    "CP A, A",
    "RET NZ",
    "POP BC",
    "JP NZ, NN",
    "JP NN",
    "CALL NZ, NN",
    "PUSH BC",
    "ADD A, N",
    "RST 00",
    "RET Z",
    "RET",
    "JP Z, NN",
    "UNDEFINED",
    "CALL Z, NN",
    "CALL NN",
    "ADC A, N",
    "RST 08",
    "RET NC",
    "POP DE",
    "JP NC, NN",
    "UNDEFINED",
    "CALL NC, NN",
    "PUSH DE",
    "SUB A, N",
    "RST 10",
    "RET C",
    "RETI",
    "JP C, NN",
    "UNDEFINED",
    "CALL C, NN",
    "UNDEFINED",
    "SBC A, N",
    "RST 18",
    "LDH (N), A",
    "POP HL",
    "LDH (C), A",
    "UNDEFINED",
    "UNDEFINED",
    "PUSH HL",
    "AND A, N",
    "RST 20",
    "ADD SP, SN",
    "JP HL",
    "LD (NN), A",
    "UNDEFINED",
    "UNDEFINED",
    "UNDEFINED",
    "XOR A, N",
    "RST 28",
    "LDH A, (N)",
    "POP AF",
    "LDH A, (C)",
    "DI",
    "UNDEFINED",
    "PUSH AF",
    "OR A, N",
    "RST 30",
    "LD HL, SP, SN",
    "LD SP, HL",
    "LD A, (NN)",
    "EI",
    "UNDEFINED",
    "UNDEFINED",
    "CP A, N",
    "RST 38",
    "RLC B",
    "RLC C",
    "RLC D",
    "RLC E",
    "RLC H",
    "RLC L",
    "RLC (HL)",
    "RLC A",
    "RRC B",
    "RRC C",
    "RRC D",
    "RRC E",
    "RRC H",
    "RRC L",
    "RRC (HL)",
    "RRC A",
    "RL B",
    "RL C",
    "RL D",
    "RL E",
    "RL H",
    "RL L",
    "RL (HL)",
    "RL A",
    "RR B",
    "RR C",
    "RR D",
    "RR E",
    "RR H",
    "RR L",
    "RR (HL)",
    "RR A",
    "SLA B",
    "SLA C",
    "SLA D",
    "SLA E",
    "SLA H",
    "SLA L",
    "SLA (HL)",
    "SLA A",
    "SRA B",
    "SRA C",
    "SRA D",
    "SRA E",
    "SRA H",
    "SRA L",
    "SRA (HL)",
    "SRA A",
    "SWAP B",
    "SWAP C",
    "SWAP D",
    "SWAP E",
    "SWAP H",
    "SWAP L",
    "SWAP (HL)",
    "SWAP A",
    "SRL B",
    "SRL C",
    "SRL D",
    "SRL E",
    "SRL H",
    "SRL L",
    "SRL (HL)",
    "SRL A",
    "BIT B, 0",
    "BIT C, 0",
    "BIT D, 0",
    "BIT E, 0",
    "BIT H, 0",
    "BIT L, 0",
    "BIT (HL), 0",
    "BIT A, 0",
    "BIT B, 1",
    "BIT C, 1",
    "BIT D, 1",
    "BIT E, 1",
    "BIT H, 1",
    "BIT L, 1",
    "BIT (HL), 1",
    "BIT A, 1",
    "BIT B, 2",
    "BIT C, 2",
    "BIT D, 2",
    "BIT E, 2",
    "BIT H, 2",
    "BIT L, 2",
    "BIT (HL), 2",
    "BIT A, 2",
    "BIT B, 3",
    "BIT C, 3",
    "BIT D, 3",
    "BIT E, 3",
    "BIT H, 3",
    "BIT L, 3",
    "BIT (HL), 3",
    "BIT A, 3",
    "BIT B, 4",
    "BIT C, 4",
    "BIT D, 4",
    "BIT E, 4",
    "BIT H, 4",
    "BIT L, 4",
    "BIT (HL), 4",
    "BIT A, 4",
    "BIT B, 5",
    "BIT C, 5",
    "BIT D, 5",
    "BIT E, 5",
    "BIT H, 5",
    "BIT L, 5",
    "BIT (HL), 5",
    "BIT A, 5",
    "BIT B, 6",
    "BIT C, 6",
    "BIT D, 6",
    "BIT E, 6",
    "BIT H, 6",
    "BIT L, 6",
    "BIT (HL), 6",
    "BIT A, 6",
    "BIT B, 7",
    "BIT C, 7",
    "BIT D, 7",
    "BIT E, 7",
    "BIT H, 7",
    "BIT L, 7",
    "BIT (HL), 7",
    "BIT A, 7",
    "RES B, 0",
    "RES C, 0",
    "RES D, 0",
    "RES E, 0",
    "RES H, 0",
    "RES L, 0",
    "RES (HL), 0",
    "RES A, 0",
    "RES B, 1",
    "RES C, 1",
    "RES D, 1",
    "RES E, 1",
    "RES H, 1",
    "RES L, 1",
    "RES (HL), 1",
    "RES A, 1",
    "RES B, 2",
    "RES C, 2",
    "RES D, 2",
    "RES E, 2",
    "RES H, 2",
    "RES L, 2",
    "RES (HL), 2",
    "RES A, 2",
    "RES B, 3",
    "RES C, 3",
    "RES D, 3",
    "RES E, 3",
    "RES H, 3",
    "RES L, 3",
    "RES (HL), 3",
    "RES A, 3",
    "RES B, 4",
    "RES C, 4",
    "RES D, 4",
    "RES E, 4",
    "RES H, 4",
    "RES L, 4",
    "RES (HL), 4",
    "RES A, 4",
    "RES B, 5",
    "RES C, 5",
    "RES D, 5",
    "RES E, 5",
    "RES H, 5",
    "RES L, 5",
    "RES (HL), 5",
    "RES A, 5",
    "RES B, 6",
    "RES C, 6",
    "RES D, 6",
    "RES E, 6",
    "RES H, 6",
    "RES L, 6",
    "RES (HL), 6",
    "RES A, 6",
    "RES B, 7",
    "RES C, 7",
    "RES D, 7",
    "RES E, 7",
    "RES H, 7",
    "RES L, 7",
    "RES (HL), 7",
    "RES A, 7",
    "SET B, 0",
    "SET C, 0",
    "SET D, 0",
    "SET E, 0",
    "SET H, 0",
    "SET L, 0",
    "SET (HL), 0",
    "SET A, 0",
    "SET B, 1",
    "SET C, 1",
    "SET D, 1",
    "SET E, 1",
    "SET H, 1",
    "SET L, 1",
    "SET (HL), 1",
    "SET A, 1",
    "SET B, 2",
    "SET C, 2",
    "SET D, 2",
    "SET E, 2",
    "SET H, 2",
    "SET L, 2",
    "SET (HL), 2",
    "SET A, 2",
    "SET B, 3",
    "SET C, 3",
    "SET D, 3",
    "SET E, 3",
    "SET H, 3",
    "SET L, 3",
    "SET (HL), 3",
    "SET A, 3",
    "SET B, 4",
    "SET C, 4",
    "SET D, 4",
    "SET E, 4",
    "SET H, 4",
    "SET L, 4",
    "SET (HL), 4",
    "SET A, 4",
    "SET B, 5",
    "SET C, 5",
    "SET D, 5",
    "SET E, 5",
    "SET H, 5",
    "SET L, 5",
    "SET (HL), 5",
    "SET A, 5",
    "SET B, 6",
    "SET C, 6",
    "SET D, 6",
    "SET E, 6",
    "SET H, 6",
    "SET L, 6",
    "SET (HL), 6",
    "SET A, 6",
    "SET B, 7",
    "SET C, 7",
    "SET D, 7",
    "SET E, 7",
    "SET H, 7",
    "SET L, 7",
    "SET (HL), 7",
    "SET A, 7",
];
