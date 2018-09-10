#[cfg(test)]
mod tests;

use byteorder::{ByteOrder, LittleEndian};
use ram::Ram;
use std::default::Default;

pub mod cpuflags;
pub mod opcodes;

#[derive(Default)]
pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: cpuflags::CpuFlags,
    pc: usize, // Actually u16 but defined as usize to avoid casting for indexing
    sp: u16,
    cycles: usize,
}

impl Cpu {
    #[inline(always)]
    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }
    #[inline(always)]
    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }
    #[inline(always)]
    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }
}
macro_rules! ld_rr_nn {
    ($cpu:expr, $ram:expr, $n1:ident, $n2:ident) => {
        $cpu.$n2 = $cpu.next_byte($ram);
        $cpu.$n1 = $cpu.next_byte($ram);
        $cpu.cycles += 12;
    };
}
macro_rules! ld_r1_r2 {
    ($cpu:expr, $r1:ident, $r2:ident) => {
        $cpu.$r1 = $cpu.$r2;
        $cpu.cycles += 4;
    };
}
macro_rules! ld_r_rr {
    ($cpu:expr, $ram:expr, $r:ident, $rr:ident) => {
        $cpu.$r = $ram[$cpu.$rr() as usize];
        $cpu.cycles += 8;
    };
}
macro_rules! ld_rr_r {
    ($cpu:expr, $ram:expr, $rr:ident, $r:ident) => {
        $ram[$cpu.$rr() as usize] = $cpu.$r;
        $cpu.cycles += 8;
    };
}
macro_rules! ld_r_n {
    ($cpu:expr, $ram:expr, $r:ident) => {
        $cpu.$r = $cpu.next_byte($ram);
        $cpu.cycles += 8;
    };
}
macro_rules! add_a_r {
    ($cpu:expr, $value:expr) => {{
        $cpu.f.unset_n();
        let check = ($cpu.a as u16) + ($value as u16);
        if check >= 0xFF {
            $cpu.f.set_h();
            $cpu.f.set_c();
        }
        let check = $cpu.a;
        $cpu.a = $cpu.a.wrapping_add($value);
        if $cpu.a == 0 {
            $cpu.f.set_z();
        }
        if cpuflags::test_half_carry_addition(check, $cpu.a) {
            $cpu.f.set_h();
        }
        $cpu.cycles += 4;
    }};
}
macro_rules! inc_rr {
    ($cpu:expr, $r1:ident, $r2:ident) => {{
        if $cpu.$r2 == 0xFF {
            $cpu.$r1 = $cpu.$r1.wrapping_add(1);
            $cpu.$r2 = $cpu.$r2.wrapping_add(1);
        } else {
            $cpu.$r2 += 0x1;
        }
        $cpu.cycles += 8;
    }};
}
macro_rules! inc_r {
    ($cpu:expr, $value:expr) => {{
        let check = $value;
        $value = $value.wrapping_add(1);
        if $value == 0x0 {
            $cpu.f.set_z();
        }
        if cpuflags::test_half_carry_addition(check, $value) {
            $cpu.f.set_h();
        }
        $cpu.f.unset_n();
        $cpu.cycles += 4;
        $value
    }};
}
macro_rules! dec_r {
    ($cpu:expr, $value:expr) => {{
        let check = $value;
        $value = $value.wrapping_sub(1);
        if $value == 0x0 {
            $cpu.f.set_z();
        }
        if !cpuflags::test_half_carry_subtraction(check, $value) {
            $cpu.f.set_h();
        }
        $cpu.f.unset_n();
        $cpu.cycles += 4;
        $value
    }};
}
macro_rules! dec_rr {
    ($cpu:expr, $r1:ident, $r2:ident) => {{
        if $cpu.$r2 == 0x0 {
            $cpu.$r1 = $cpu.$r1.wrapping_sub(1);
            $cpu.$r2 = $cpu.$r2.wrapping_sub(1);
        } else {
            $cpu.$r2 -= 0x1;
        }
        $cpu.cycles += 8;
    }};
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }
    pub fn _reset(&mut self) {
        self.sp = 0xFFFE;
        self.pc = 0x100;
    }
    fn next_byte(&mut self, ram: &mut Ram) -> u8 {
        if self.pc > 65536 {
            panic!("Program counter out of bounds")
        }
        let ret = ram[self.pc];
        self.pc += 1;
        ret
    }

    fn nyi(&mut self, _: &mut Ram) {
        panic!("Instruction not yet implemented")
    }
    fn nop(&mut self, _: &mut Ram) {
        self.cycles += 8;
    }

    fn add_a_a(&mut self, _ram: &mut Ram) {
        add_a_r!(self, self.a);
    }
    fn add_a_b(&mut self, _ram: &mut Ram) {
        add_a_r!(self, self.b);
    }
    fn add_a_c(&mut self, _ram: &mut Ram) {
        add_a_r!(self, self.c);
    }
    fn add_a_d(&mut self, _ram: &mut Ram) {
        add_a_r!(self, self.d);
    }
    fn add_a_e(&mut self, _ram: &mut Ram) {
        add_a_r!(self, self.e);
    }
    fn add_a_h(&mut self, _ram: &mut Ram) {
        add_a_r!(self, self.h);
    }
    fn add_a_l(&mut self, _ram: &mut Ram) {
        add_a_r!(self, self.l);
    }
    fn add_a_hl(&mut self, ram: &mut Ram) {
        add_a_r!(self, ram[self.hl() as usize]);
        self.cycles += 4;
    }

    fn inc_a(&mut self, _ram: &mut Ram) {
        self.a = inc_r!(self, self.a);
    }
    fn inc_b(&mut self, _ram: &mut Ram) {
        self.b = inc_r!(self, self.b);
    }
    fn inc_c(&mut self, _ram: &mut Ram) {
        self.c = inc_r!(self, self.c);
    }
    fn inc_d(&mut self, _ram: &mut Ram) {
        self.d = inc_r!(self, self.d);
    }
    fn inc_e(&mut self, _ram: &mut Ram) {
        self.e = inc_r!(self, self.e);
    }
    fn inc_h(&mut self, _ram: &mut Ram) {
        self.h = inc_r!(self, self.h);
    }
    fn inc_l(&mut self, _ram: &mut Ram) {
        self.l = inc_r!(self, self.l);
    }
    fn inc_hl(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = inc_r!(self, ram[self.hl() as usize]);
        self.cycles += 8;
    }

    fn dec_a(&mut self, _ram: &mut Ram) {
        self.a = dec_r!(self, self.a);
    }
    fn dec_b(&mut self, _ram: &mut Ram) {
        self.b = dec_r!(self, self.b);
    }
    fn dec_c(&mut self, _ram: &mut Ram) {
        self.c = dec_r!(self, self.c);
    }
    fn dec_d(&mut self, _ram: &mut Ram) {
        self.d = dec_r!(self, self.d);
    }
    fn dec_e(&mut self, _ram: &mut Ram) {
        self.e = dec_r!(self, self.e);
    }
    fn dec_h(&mut self, _ram: &mut Ram) {
        self.h = dec_r!(self, self.h);
    }
    fn dec_l(&mut self, _ram: &mut Ram) {
        self.l = dec_r!(self, self.l);
    }
    fn dec_hl(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = dec_r!(self, ram[self.hl() as usize]);
        self.cycles += 8;
    }

    fn dec_combined_bc(&mut self, _ram: &mut Ram) {
        dec_rr!(self, b, c)
    }
    fn dec_combined_de(&mut self, _ram: &mut Ram) {
        dec_rr!(self, d, e)
    }
    fn dec_combined_hl(&mut self, _ram: &mut Ram) {
        dec_rr!(self, h, l)
    }
    fn dec_combined_sp(&mut self, _ram: &mut Ram) {
        self.sp = self.sp.wrapping_sub(1);
        self.cycles += 8;
    }
    fn inc_combined_bc(&mut self, _ram: &mut Ram) {
        inc_rr!(self, b, c)
    }
    fn inc_combined_de(&mut self, _ram: &mut Ram) {
        inc_rr!(self, d, e)
    }
    fn inc_combined_hl(&mut self, _ram: &mut Ram) {
        inc_rr!(self, h, l)
    }
    fn inc_combined_sp(&mut self, _ram: &mut Ram) {
        self.sp = self.sp.wrapping_add(1);
        self.cycles += 8;
    }

    fn ld_bc_a(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, bc, a);
    }
    fn ld_bc_nn(&mut self, ram: &mut Ram) {
        ld_rr_nn!(self, ram, b, c);
    }
    fn ld_de_a(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, de, a);
    }
    fn ld_de_nn(&mut self, ram: &mut Ram) {
        ld_rr_nn!(self, ram, d, e);
    }
    fn ld_nn_a(&mut self, ram: &mut Ram) {
        let mut addr = 0u16;
        addr |= self.next_byte(ram) as u16;
        addr |= (self.next_byte(ram) as u16) << 8;
        ram[addr as usize] = self.a;
        self.cycles += 16;
    }
    fn ld_hl_nn(&mut self, ram: &mut Ram) {
        ld_rr_nn!(self, ram, h, l);
    }
    fn ld_sp_nn(&mut self, ram: &mut Ram) {
        self.sp = LittleEndian::read_u16(&ram[self.pc..]);
        self.pc += 2;
        self.cycles += 12;
    }

    fn ld_a_a(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, a, a);
    }
    fn ld_a_b(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, a, b);
    }
    fn ld_a_c(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, a, c);
    }
    fn ld_a_d(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, a, d);
    }
    fn ld_a_e(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, a, e);
    }
    fn ld_a_h(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, a, h);
    }
    fn ld_a_l(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, a, l);
    }
    fn ld_a_hl(&mut self, ram: &mut Ram) {
        ld_r_rr!(self, ram, a, hl);
    }
    fn ld_a_bc(&mut self, ram: &mut Ram) {
        self.a = ram[self.bc() as usize];
        self.cycles += 8;
    }
    fn ld_a_de(&mut self, ram: &mut Ram) {
        self.a = ram[self.de() as usize];
        self.cycles += 8;
    }
    fn ld_a_nn(&mut self, ram: &mut Ram) {
        let mut addr = 0u16;
        addr |= self.next_byte(ram) as u16;
        addr |= (self.next_byte(ram) as u16) << 8;
        self.a = ram[addr as usize];
        self.cycles += 16;
    }
    fn ld_a_addr_c(&mut self, ram: &mut Ram) {
        self.a = ram[0xFF00 | (self.c as usize)];
        self.cycles += 8;
    }
    fn ld_addr_c_a(&mut self, ram: &mut Ram) {
        ram[0xFF00 | (self.c as usize)] = self.a;
        self.cycles += 8;
    }

    fn ld_b_a(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, b, a);
    }
    fn ld_b_b(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, b, b);
    }
    fn ld_b_c(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, b, c);
    }
    fn ld_b_d(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, b, d);
    }
    fn ld_b_e(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, b, e);
    }
    fn ld_b_h(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, b, h);
    }
    fn ld_b_l(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, b, l);
    }
    fn ld_b_hl(&mut self, ram: &mut Ram) {
        ld_r_rr!(self, ram, b, hl);
    }

    fn ld_c_a(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, c, a);
    }
    fn ld_c_b(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, c, b);
    }
    fn ld_c_c(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, c, c);
    }
    fn ld_c_d(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, c, d);
    }
    fn ld_c_e(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, c, e);
    }
    fn ld_c_h(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, c, h);
    }
    fn ld_c_l(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, c, l);
    }
    fn ld_c_hl(&mut self, ram: &mut Ram) {
        ld_r_rr!(self, ram, c, hl);
    }

    fn ld_d_a(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, d, a);
    }
    fn ld_d_b(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, d, b);
    }
    fn ld_d_c(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, d, c);
    }
    fn ld_d_d(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, d, d);
    }
    fn ld_d_e(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, d, e);
    }
    fn ld_d_h(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, d, h);
    }
    fn ld_d_l(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, d, l);
    }
    fn ld_d_hl(&mut self, ram: &mut Ram) {
        ld_r_rr!(self, ram, d, hl);
    }

    fn ld_e_a(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, e, a);
    }
    fn ld_e_b(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, e, b);
    }
    fn ld_e_c(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, e, c);
    }
    fn ld_e_d(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, e, d);
    }
    fn ld_e_e(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, e, e);
    }
    fn ld_e_h(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, e, h);
    }
    fn ld_e_l(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, e, l);
    }
    fn ld_e_hl(&mut self, ram: &mut Ram) {
        ld_r_rr!(self, ram, e, hl);
    }

    fn ld_h_a(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, h, a);
    }
    fn ld_h_b(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, h, b);
    }
    fn ld_h_c(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, h, c);
    }
    fn ld_h_d(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, h, d);
    }
    fn ld_h_e(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, h, e);
    }
    fn ld_h_h(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, h, h);
    }
    fn ld_h_l(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, h, l);
    }
    fn ld_h_hl(&mut self, ram: &mut Ram) {
        ld_r_rr!(self, ram, h, hl);
    }

    fn ld_l_a(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, l, a);
    }
    fn ld_l_b(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, l, b);
    }
    fn ld_l_c(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, l, c);
    }
    fn ld_l_d(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, l, d);
    }
    fn ld_l_e(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, l, e);
    }
    fn ld_l_h(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, l, h);
    }
    fn ld_l_l(&mut self, _ram: &mut Ram) {
        ld_r1_r2!(self, l, l);
    }
    fn ld_l_hl(&mut self, ram: &mut Ram) {
        ld_r_rr!(self, ram, l, hl);
    }

    fn ld_hl_a(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, hl, a);
    }
    fn ld_hl_b(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, hl, b);
    }
    fn ld_hl_c(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, hl, c);
    }
    fn ld_hl_d(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, hl, d);
    }
    fn ld_hl_e(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, hl, e);
    }
    fn ld_hl_h(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, hl, h);
    }
    fn ld_hl_l(&mut self, ram: &mut Ram) {
        ld_rr_r!(self, ram, hl, l);
    }
    fn ld_hl_n(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = self.next_byte(ram);
        self.cycles += 12;
    }

    fn ld_a_n(&mut self, ram: &mut Ram) {
        ld_r_n!(self, ram, a);
    }
    fn ld_b_n(&mut self, ram: &mut Ram) {
        ld_r_n!(self, ram, b);
    }
    fn ld_c_n(&mut self, ram: &mut Ram) {
        ld_r_n!(self, ram, c);
    }
    fn ld_d_n(&mut self, ram: &mut Ram) {
        ld_r_n!(self, ram, d);
    }
    fn ld_e_n(&mut self, ram: &mut Ram) {
        ld_r_n!(self, ram, e);
    }
    fn ld_h_n(&mut self, ram: &mut Ram) {
        ld_r_n!(self, ram, h);
    }
    fn ld_l_n(&mut self, ram: &mut Ram) {
        ld_r_n!(self, ram, l);
    }
}
