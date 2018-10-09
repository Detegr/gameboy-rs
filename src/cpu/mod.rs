#[cfg(test)]
mod tests;

use byteorder::{ByteOrder, LittleEndian};
use ram::Ram;
use std::default::Default;

pub mod cpuflags;
pub mod opcodes;

#[derive(Debug, PartialEq)]
pub enum RunState {
    Running,
    Stopped,
    Halted,
}
impl Default for RunState {
    fn default() -> RunState {
        RunState::Running
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InterruptState {
    Disabled,
    Enabled,
    WillDisable,
    WillEnable,
}
impl Default for InterruptState {
    fn default() -> InterruptState {
        InterruptState::Enabled
    }
}

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
    pc: u16,
    sp: u16,
    run_state: RunState,
    interrupts: InterruptState,
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
    #[inline]
    fn push(&mut self, ram: &mut Ram, value: u8) {
        assert!(self.sp > 0);
        ram[self.sp as usize] = value;
        self.sp -= 1;
    }
    #[inline]
    fn push_u16(&mut self, ram: &mut Ram, value: u16) {
        self.push(ram, (value & 0xFF) as u8);
        self.push(ram, (value >> 8) as u8);
    }
}
macro_rules! make_ld_rr_nn {
    ($name: ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            self.$r2 = self.next_byte(ram);
            self.$r1 = self.next_byte(ram);
            self.cycles += 12;
        }
    }
}
macro_rules! make_ld_r_r {
    ($name:ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            self.$r1 = self.$r2;
            self.cycles += 4;
        }
    }
}
macro_rules! make_ld_r_rr {
    ($name: ident, $r:ident, $rr:ident) => {
        fn $name(&mut self, ram: &mut Ram) {
            self.$r = ram[self.$rr() as usize];
            self.cycles += 8;
        }
    }
}
macro_rules! make_ld_rr_r {
    ($name: ident, $rr:ident, $r:ident) => {
        fn $name(&mut self, ram: &mut Ram) {
            ram[self.$rr() as usize] = self.$r;
            self.cycles += 8;
        }
    }
}
macro_rules! make_ld_r_n {
    ($name: ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            self.$r = self.next_byte(ram);
            self.cycles += 8;
        }
    }
}
macro_rules! make_add {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            add_a_n!(self, self.$reg);
        }
    }
}
macro_rules! make_adc {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            if self.f.c() {
                add_a_n!(self, self.$reg.wrapping_add(1));
            } else {
                add_a_n!(self, self.$reg);
            }
        }
    }
}
macro_rules! make_add_rr_rr {
    ($name:ident, $r1: ident, $r2:ident, $r3:ident, $r4:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            let val32_lhs = ((self.$r1 as u32) << 8) & 0xFF00 | self.$r2 as u32 & 0xFF;
            let val32_rhs = ((self.$r3 as u32) << 8) & 0xFF00 | self.$r4 as u32 & 0xFF;
            let mut r2 = self.$r2;
            r2 = r2.wrapping_add(self.$r4);
            let r1_add = if r2 < self.$r2 { 1 } else { 0 };
            let mut r1 = self.$r1;
            r1 = r1.wrapping_add(self.$r3 + r1_add);
            if cpuflags::test_half_carry_addition(self.$r1, self.$r3 + r1_add) {
                self.f.set_h();
            } else {
                self.f.unset_h();
            }
            if val32_lhs + val32_rhs > 65535 {
                self.f.set_c();
                self.f.set_h();
            } else {
                self.f.unset_c();
            }
            self.f.unset_n();
            self.cycles += 8;

            self.$r1 = r1;
            self.$r2 = r2;
        }
    }
}
macro_rules! add_a_n {
    ($cpu:expr, $value:expr) => {{
        $cpu.f.unset_n();
        let check = ($cpu.a as u16) + ($value as u16);
        if check >= 0xFF {
            $cpu.f.set_h();
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_h();
            $cpu.f.unset_c();
        }
        if cpuflags::test_half_carry_addition($cpu.a, $value) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        $cpu.a = $cpu.a.wrapping_add($value);
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    }};
}
macro_rules! make_sub {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            sub_a_r!(self, self.$reg);
        }
    }
}
macro_rules! make_sbc {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            if self.f.c() {
                sub_a_r!(self, self.$reg.wrapping_add(1));
            } else {
                sub_a_r!(self, self.$reg);
            }
        }
    }
}
macro_rules! sub_a_r {
    ($cpu:expr, $value:expr) => {{
        $cpu.f.set_n();
        let check = ($cpu.a as i16) - ($value as i16);
        if check < 0 {
            $cpu.f.set_h();
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_h();
            $cpu.f.unset_c();
        }
        if cpuflags::test_half_carry_subtraction($cpu.a, $value) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        $cpu.a = $cpu.a.wrapping_sub($value);
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    }};
}
macro_rules! make_inc_rr {
    ($name:ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            if self.$r2 == 0xFF {
                self.$r1 = self.$r1.wrapping_add(1);
                self.$r2 = self.$r2.wrapping_add(1);
            } else {
                self.$r2 += 0x1;
            }
            self.cycles += 8;
        }
    }
}
macro_rules! make_inc {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            self.$reg = inc_r!(self, self.$reg);
        }
    }
}
macro_rules! inc_r {
    ($cpu:expr, $value:expr) => {{
        if cpuflags::test_half_carry_addition($value, 1) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        $value = $value.wrapping_add(1);
        if $value == 0x0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.f.unset_n();
        $cpu.cycles += 4;
        $value
    }};
}
macro_rules! make_dec {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            self.$reg = dec_r!(self, self.$reg);
        }
    }
}
macro_rules! dec_r {
    ($cpu:expr, $value:expr) => {{
        if cpuflags::test_half_carry_subtraction($value, 1) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        $value = $value.wrapping_sub(1);
        if $value == 0x0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.f.set_n();
        $cpu.cycles += 4;
        $value
    }};
}
macro_rules! make_dec_rr {
    ($name:ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            if self.$r2 == 0x0 {
                self.$r1 = self.$r1.wrapping_sub(1);
                self.$r2 = self.$r2.wrapping_sub(1);
            } else {
                self.$r2 -= 0x1;
            }
            self.cycles += 8;
        }
    }
}
macro_rules! make_jr_cc_n {
    ($name:ident, $flag:ident set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if self.f.$flag() {
                self.jr_n(ram);
            } else {
                self.pc += 1;
                self.cycles += 8;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if !self.f.$flag() {
                self.jr_n(ram);
            } else {
                self.pc += 1;
                self.cycles += 8;
            }
        }
    }
}
macro_rules! make_and {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            self.f.set_h();
            self.f.unset_n();
            self.f.unset_c();
            self.a &= self.$r;
            if self.a == 0 {
                self.f.set_z();
            } else {
                self.f.unset_z();
            }
            self.cycles += 4;
        }
    }
}
macro_rules! make_or {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            self.f.unset_h();
            self.f.unset_n();
            self.f.unset_c();
            self.a |= self.$r;
            if self.a == 0 {
                self.f.set_z();
            } else {
                self.f.unset_z();
            }
            self.cycles += 4;
        }
    }
}
macro_rules! make_xor {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            self.f.unset_h();
            self.f.unset_n();
            self.f.unset_c();
            self.a ^= self.$r;
            if self.a == 0 {
                self.f.set_z();
            } else {
                self.f.unset_z();
            }
            self.cycles += 4;
        }
    }
}
macro_rules! cp_a_r {
    ($cpu:expr, $value:expr) => {{
        $cpu.f.set_n();
        let check = ($cpu.a as i16) - ($value as i16);
        if check < 0 {
            $cpu.f.set_h();
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_h();
            $cpu.f.unset_c();
        }
        if cpuflags::test_half_carry_subtraction($cpu.a, $value) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        if $cpu.a == $value {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    }};
}
macro_rules! make_cp {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _ram: &mut Ram) {
            cp_a_r!(self, self.$r)
        }
    }
}
macro_rules! make_ret {
    ($name:ident, $flag:ident set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if self.f.$flag() {
                self.ret(ram);
                self.cycles += 4;
            } else {
                self.cycles += 8;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if !self.f.$flag() {
                self.ret(ram);
                self.cycles += 4;
            } else {
                self.cycles += 8;
            }
        }
    }
}
macro_rules! make_pop {
    ($name:ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            assert!(
                self.sp.wrapping_add(2) > self.sp,
                "less than 2 bytes of data in the stack"
            );
            self.sp += 1;
            let byte = ram[self.sp as usize];
            self.$r2 = byte;
            self.sp += 1;
            let byte = ram[self.sp as usize];
            self.$r1 = byte;

            self.cycles += 12;
        }
    }
}
macro_rules! make_jp {
    ($name:ident, $flag:ident set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if self.f.$flag() {
                self.jp(ram);
            } else {
                self.cycles += 12;
                self.pc += 2;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if !self.f.$flag() {
                self.jp(ram);
            } else {
                self.cycles += 12;
                self.pc += 2;
            }
        }
    }
}
macro_rules! make_call {
    ($name:ident, $flag:ident set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if self.f.$flag() {
                self.call(ram);
            } else {
                self.pc += 2;
                self.cycles += 12;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            if !self.f.$flag() {
                self.call(ram);
            } else {
                self.pc += 2;
                self.cycles += 12;
            }
        }
    }
}
macro_rules! make_push {
    ($name:ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            assert!(
                self.sp.wrapping_sub(2) < self.sp,
                "stack overflow"
            );

            let v1 = self.$r1;
            let v2 = self.$r2;

            self.push(ram, v1);
            self.push(ram, v2);

            self.cycles += 16;
        }
    }
}
macro_rules! make_rst {
    ($name:ident, $to:expr) => {
        #[inline]
        fn $name(&mut self, ram: &mut Ram) {
            let val = self.pc;
            self.push_u16(ram, val);
            self.pc = $to;
            self.cycles += 16;
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }
    #[inline]
    pub fn reset(&mut self) {
        self.sp = 0xFFFE;
        self.pc = 0x100;
        self.interrupts = InterruptState::Enabled; // TODO: This is just a guess
    }
    pub fn step(&mut self, ram: &mut Ram) {
        let opcode = {
            let opcode = self.next_byte(ram);
            if opcode == 0xCB {
                self.next_byte(ram) + 0xFF
            } else {
                opcode
            }
        };
        let old_interrupts_state = self.interrupts;
        let new_interrupts_state = match self.interrupts {
            InterruptState::WillEnable => InterruptState::Enabled,
            InterruptState::WillDisable => InterruptState::Disabled,
            state => state,
        };
        opcodes::OPCODES[opcode as usize](self, ram);
        if self.interrupts == old_interrupts_state {
            // The instruction did not modify interrupts flag,
            // enable/disable if we were in WillEnable/WillDisable state
            self.interrupts = new_interrupts_state;
        }
    }
    #[inline]
    fn next_byte(&mut self, ram: &mut Ram) -> u8 {
        let ret = ram[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        ret
    }

    fn nyi(&mut self, _: &mut Ram) {
        panic!("Instruction not yet implemented")
    }

    fn na(&mut self, _: &mut Ram) {
        panic!("Instruction not available. This is a bug.")
    }

    #[inline]
    fn nop(&mut self, _: &mut Ram) {
        self.cycles += 8;
    }

    make_add!(add_a_a, a);
    make_add!(add_a_b, b);
    make_add!(add_a_c, c);
    make_add!(add_a_d, d);
    make_add!(add_a_e, e);
    make_add!(add_a_h, h);
    make_add!(add_a_l, l);

    #[inline]
    fn add_a_deref_hl(&mut self, ram: &mut Ram) {
        add_a_n!(self, ram[self.hl() as usize]);
        self.cycles += 4;
    }

    make_inc!(inc_a, a);
    make_inc!(inc_b, b);
    make_inc!(inc_c, c);
    make_inc!(inc_d, d);
    make_inc!(inc_e, e);
    make_inc!(inc_h, h);
    make_inc!(inc_l, l);

    #[inline]
    fn inc_deref_hl(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = inc_r!(self, ram[self.hl() as usize]);
        self.cycles += 8;
    }

    make_dec!(dec_a, a);
    make_dec!(dec_b, b);
    make_dec!(dec_c, c);
    make_dec!(dec_d, d);
    make_dec!(dec_e, e);
    make_dec!(dec_h, h);
    make_dec!(dec_l, l);

    #[inline]
    fn dec_deref_hl(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = dec_r!(self, ram[self.hl() as usize]);
        self.cycles += 8;
    }

    make_dec_rr!(dec_bc, b, c);
    make_dec_rr!(dec_de, d, e);
    make_dec_rr!(dec_hl, h, l);
    #[inline]
    fn dec_sp(&mut self, _ram: &mut Ram) {
        self.sp = self.sp.wrapping_sub(1);
        self.cycles += 8;
    }
    make_inc_rr!(inc_bc, b, c);
    make_inc_rr!(inc_de, d, e);
    make_inc_rr!(inc_hl, h, l);
    #[inline]
    fn inc_sp(&mut self, _ram: &mut Ram) {
        self.sp = self.sp.wrapping_add(1);
        self.cycles += 8;
    }

    make_ld_rr_r!(ld_bc_a, bc, a);
    make_ld_rr_nn!(ld_bc_nn, b, c);
    make_ld_rr_r!(ld_de_a, de, a);
    make_ld_rr_nn!(ld_de_nn, d, e);
    #[inline]
    fn ld_nn_a(&mut self, ram: &mut Ram) {
        let mut addr = 0u16;
        addr |= self.next_byte(ram) as u16;
        addr |= (self.next_byte(ram) as u16) << 8;
        ram[addr as usize] = self.a;
        self.cycles += 16;
    }
    make_ld_rr_nn!(ld_hl_nn, h, l);
    #[inline]
    fn ld_sp_nn(&mut self, ram: &mut Ram) {
        self.sp = LittleEndian::read_u16(&ram[self.pc as usize..]);
        self.pc += 2;
        self.cycles += 12;
    }

    make_ld_r_r!(ld_a_a, a, a);
    make_ld_r_r!(ld_a_b, a, b);
    make_ld_r_r!(ld_a_c, a, c);
    make_ld_r_r!(ld_a_d, a, d);
    make_ld_r_r!(ld_a_e, a, e);
    make_ld_r_r!(ld_a_h, a, h);
    make_ld_r_r!(ld_a_l, a, l);

    make_ld_r_rr!(ld_a_deref_hl, a, hl);
    make_ld_r_rr!(ld_a_deref_bc, a, bc);
    make_ld_r_rr!(ld_a_deref_de, a, de);

    #[inline]
    fn ld_a_nn(&mut self, ram: &mut Ram) {
        let mut addr = 0u16;
        addr |= self.next_byte(ram) as u16;
        addr |= (self.next_byte(ram) as u16) << 8;
        self.a = ram[addr as usize];
        self.cycles += 16;
    }
    #[inline]
    fn ld_a_addr_c(&mut self, ram: &mut Ram) {
        self.a = ram[0xFF00 | (self.c as usize)];
        self.cycles += 8;
    }
    #[inline]
    fn ld_addr_c_a(&mut self, ram: &mut Ram) {
        ram[0xFF00 | (self.c as usize)] = self.a;
        self.cycles += 8;
    }

    make_ld_r_r!(ld_b_a, b, a);
    make_ld_r_r!(ld_b_b, b, b);
    make_ld_r_r!(ld_b_c, b, c);
    make_ld_r_r!(ld_b_d, b, d);
    make_ld_r_r!(ld_b_e, b, e);
    make_ld_r_r!(ld_b_h, b, h);
    make_ld_r_r!(ld_b_l, b, l);

    make_ld_r_rr!(ld_b_deref_hl, b, hl);

    make_ld_r_r!(ld_c_a, c, a);
    make_ld_r_r!(ld_c_b, c, b);
    make_ld_r_r!(ld_c_c, c, c);
    make_ld_r_r!(ld_c_d, c, d);
    make_ld_r_r!(ld_c_e, c, e);
    make_ld_r_r!(ld_c_h, c, h);
    make_ld_r_r!(ld_c_l, c, l);

    make_ld_r_rr!(ld_c_deref_hl, c, hl);

    make_ld_r_r!(ld_d_a, d, a);
    make_ld_r_r!(ld_d_b, d, b);
    make_ld_r_r!(ld_d_c, d, c);
    make_ld_r_r!(ld_d_d, d, d);
    make_ld_r_r!(ld_d_e, d, e);
    make_ld_r_r!(ld_d_h, d, h);
    make_ld_r_r!(ld_d_l, d, l);

    make_ld_r_rr!(ld_d_deref_hl, d, hl);

    make_ld_r_r!(ld_e_a, e, a);
    make_ld_r_r!(ld_e_b, e, b);
    make_ld_r_r!(ld_e_c, e, c);
    make_ld_r_r!(ld_e_d, e, d);
    make_ld_r_r!(ld_e_e, e, e);
    make_ld_r_r!(ld_e_h, e, h);
    make_ld_r_r!(ld_e_l, e, l);

    make_ld_r_rr!(ld_e_deref_hl, e, hl);

    make_ld_r_r!(ld_h_a, h, a);
    make_ld_r_r!(ld_h_b, h, b);
    make_ld_r_r!(ld_h_c, h, c);
    make_ld_r_r!(ld_h_d, h, d);
    make_ld_r_r!(ld_h_e, h, e);
    make_ld_r_r!(ld_h_h, h, h);
    make_ld_r_r!(ld_h_l, h, l);

    make_ld_r_rr!(ld_h_deref_hl, h, hl);

    make_ld_r_r!(ld_l_a, l, a);
    make_ld_r_r!(ld_l_b, l, b);
    make_ld_r_r!(ld_l_c, l, c);
    make_ld_r_r!(ld_l_d, l, d);
    make_ld_r_r!(ld_l_e, l, e);
    make_ld_r_r!(ld_l_h, l, h);
    make_ld_r_r!(ld_l_l, l, l);

    make_ld_r_rr!(ld_l_deref_hl, l, hl);

    make_ld_rr_r!(ld_hl_a, hl, a);
    make_ld_rr_r!(ld_hl_b, hl, b);
    make_ld_rr_r!(ld_hl_c, hl, c);
    make_ld_rr_r!(ld_hl_d, hl, d);
    make_ld_rr_r!(ld_hl_e, hl, e);
    make_ld_rr_r!(ld_hl_h, hl, h);
    make_ld_rr_r!(ld_hl_l, hl, l);

    #[inline]
    fn ld_deref_hl_n(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = self.next_byte(ram);
        self.cycles += 12;
    }

    make_ld_r_n!(ld_a_n, a);
    make_ld_r_n!(ld_b_n, b);
    make_ld_r_n!(ld_c_n, c);
    make_ld_r_n!(ld_d_n, d);
    make_ld_r_n!(ld_e_n, e);
    make_ld_r_n!(ld_h_n, h);
    make_ld_r_n!(ld_l_n, l);

    #[inline]
    fn rlca(&mut self, _ram: &mut Ram) {
        self.f.unset_n();
        self.f.unset_h();
        self.f.unset_z();
        if (self.a & 0x80) != 0 {
            self.f.set_c();
        } else {
            self.f.unset_c();
        }

        self.a = self.a.rotate_left(1);
        self.cycles += 4;
    }

    #[inline]
    fn rla(&mut self, _ram: &mut Ram) {
        self.f.unset_n();
        self.f.unset_h();
        self.f.unset_z();
        let old_carry = self.f.c();
        if (self.a & 0x80) != 0 {
            self.f.set_c();
        } else {
            self.f.unset_c();
        }
        self.a <<= 1;
        if old_carry {
            self.a |= 0x1;
        }
        self.cycles += 4;
    }

    #[inline]
    fn rrca(&mut self, _ram: &mut Ram) {
        self.f.unset_n();
        self.f.unset_h();
        self.f.unset_z();
        if (self.a & 0x1) != 0 {
            self.f.set_c();
        } else {
            self.f.unset_c();
        }

        self.a = self.a.rotate_right(1);
        self.cycles += 4;
    }

    #[inline]
    fn rra(&mut self, _ram: &mut Ram) {
        self.f.unset_n();
        self.f.unset_h();
        self.f.unset_z();
        let old_carry = self.f.c();
        if (self.a & 0x1) != 0 {
            self.f.set_c();
        } else {
            self.f.unset_c();
        }
        self.a >>= 1;
        if old_carry {
            self.a |= 0x80;
        }
        self.cycles += 4;
    }

    #[inline]
    fn ld_deref_a16_sp(&mut self, ram: &mut Ram) {
        let a16 = LittleEndian::read_u16(&ram[self.pc as usize..]);
        self.pc += 2;
        LittleEndian::write_u16(&mut ram[a16 as usize..], self.sp);
        self.cycles += 20;
    }

    make_add_rr_rr!(add_hl_bc, h, l, b, c);
    make_add_rr_rr!(add_hl_de, h, l, d, e);
    make_add_rr_rr!(add_hl_hl, h, l, h, l);
    #[inline]
    fn add_hl_sp(&mut self, _ram: &mut Ram) {
        let val32_lhs = ((self.h as u32) << 8) & 0xFF00 | self.l as u32 & 0xFF;
        let s = ((self.sp & 0xFF00) >> 8) as u8;
        let p = (self.sp & 0xFF) as u8;
        let mut l = self.l;
        l = l.wrapping_add(p);
        let h_add = if l < self.l { 1 } else { 0 };
        let mut h = self.h;
        h = h.wrapping_add(s + h_add);
        if cpuflags::test_half_carry_addition(self.h, s + h_add) {
            self.f.set_h();
        } else {
            self.f.unset_h();
        }
        if val32_lhs + (self.sp as u32) > 65535 {
            self.f.set_c();
            self.f.set_h();
        } else {
            self.f.unset_c();
        }
        self.f.unset_n();

        self.h = h;
        self.l = l;
        self.cycles += 8;
    }

    #[inline]
    fn jr_n(&mut self, ram: &mut Ram) {
        let n = self.next_byte(ram);

        // FIXME: How should pc behave? Is the addition done
        // after increasing it when reading a byte, or is it
        // relative to the position before reading the byte?
        self.pc = self.pc.wrapping_sub(1);
        self.pc = self.pc.wrapping_add(n as i8 as u16);

        self.cycles += 12;
    }

    make_jr_cc_n!(jr_nz_n, z not set);
    make_jr_cc_n!(jr_z_n, z set);
    make_jr_cc_n!(jr_nc_n, c not set);
    make_jr_cc_n!(jr_c_n, c set);

    #[inline]
    fn stop(&mut self, _ram: &mut Ram) {
        self.run_state = RunState::Stopped;
        self.cycles += 4;
        // TODO: Wake up to a button press
    }

    #[inline]
    fn ld_hli_a(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = self.a;
        let hl = self.hl().wrapping_add(1);
        self.h = ((hl >> 8) & 0xFF) as u8;
        self.l = (hl & 0xFF) as u8;
        self.cycles += 8;
    }

    #[inline]
    fn ld_hld_a(&mut self, ram: &mut Ram) {
        ram[self.hl() as usize] = self.a;
        let hl = self.hl().wrapping_sub(1);
        self.h = ((hl >> 8) & 0xFF) as u8;
        self.l = (hl & 0xFF) as u8;
        self.cycles += 8;
    }

    #[inline]
    fn ld_a_hli(&mut self, ram: &mut Ram) {
        self.a = ram[self.hl() as usize];
        let hl = self.hl().wrapping_add(1);
        self.h = ((hl >> 8) & 0xFF) as u8;
        self.l = (hl & 0xFF) as u8;
        self.cycles += 8;
    }

    #[inline]
    fn ld_a_hld(&mut self, ram: &mut Ram) {
        self.a = ram[self.hl() as usize];
        let hl = self.hl().wrapping_sub(1);
        self.h = ((hl >> 8) & 0xFF) as u8;
        self.l = (hl & 0xFF) as u8;
        self.cycles += 8;
    }

    fn daa(&mut self, _ram: &mut Ram) {
        if !self.f.n() {
            // Last operation was addition
            if self.f.c() || self.a > 0x99 {
                self.a = self.a.wrapping_add(0x60);
                self.f.set_c();
            }
            if self.f.h() || (self.a & 0x0F) > 0x9 {
                self.a = self.a.wrapping_add(0x6);
            }
        } else {
            // Last operation was subtraction
            if self.f.c() {
                self.a = self.a.wrapping_sub(0x60);
            }
            if self.f.h() {
                self.a = self.a.wrapping_sub(0x6);
            }
        }
        self.f.unset_h();
        if self.a == 0 {
            self.f.set_z();
        }
        self.cycles += 4;
    }

    fn cpl(&mut self, _ram: &mut Ram) {
        self.a = !self.a;
        self.f.set_n();
        self.f.set_h();

        self.cycles += 4;
    }

    fn scf(&mut self, _ram: &mut Ram) {
        self.f.unset_n();
        self.f.unset_h();
        self.f.set_c();

        self.cycles += 4;
    }

    fn ccf(&mut self, _ram: &mut Ram) {
        self.f.unset_n();
        self.f.unset_h();
        if self.f.c() {
            self.f.unset_c();
        } else {
            self.f.set_c();
        }

        self.cycles += 4;
    }

    fn halt(&mut self, _ram: &mut Ram) {
        self.run_state = RunState::Halted;
        self.cycles += 4;
        // TODO: Wake up to an interrupt
    }

    make_adc!(adc_a_b, b);
    make_adc!(adc_a_c, c);
    make_adc!(adc_a_d, d);
    make_adc!(adc_a_e, e);
    make_adc!(adc_a_h, h);
    make_adc!(adc_a_l, l);

    #[inline]
    fn adc_a_deref_hl(&mut self, ram: &mut Ram) {
        let c = if self.f.c() { 1 } else { 0 };
        add_a_n!(self, ram[self.hl() as usize].wrapping_add(c));
        self.cycles += 4;
    }

    make_adc!(adc_a_a, a);

    make_sub!(sub_a_b, b);
    make_sub!(sub_a_c, c);
    make_sub!(sub_a_d, d);
    make_sub!(sub_a_e, e);
    make_sub!(sub_a_h, h);
    make_sub!(sub_a_l, l);
    make_sub!(sub_a_a, a);

    #[inline]
    fn sub_a_deref_hl(&mut self, ram: &mut Ram) {
        sub_a_r!(self, ram[self.hl() as usize]);
        self.cycles += 4;
    }

    make_sbc!(sbc_a_b, b);
    make_sbc!(sbc_a_c, c);
    make_sbc!(sbc_a_d, d);
    make_sbc!(sbc_a_e, e);
    make_sbc!(sbc_a_h, h);
    make_sbc!(sbc_a_l, l);
    make_sbc!(sbc_a_a, a);

    #[inline]
    fn sbc_a_deref_hl(&mut self, ram: &mut Ram) {
        let c = if self.f.c() { 1 } else { 0 };
        sub_a_r!(self, ram[self.hl() as usize].wrapping_add(c));
        self.cycles += 4;
    }

    make_and!(and_a_b, b);
    make_and!(and_a_c, c);
    make_and!(and_a_d, d);
    make_and!(and_a_e, e);
    make_and!(and_a_h, h);
    make_and!(and_a_l, l);
    make_and!(and_a_a, a);

    #[inline]
    fn and_a_deref_hl(&mut self, ram: &mut Ram) {
        let value = ram[self.hl() as usize];
        self.f.set_h();
        self.f.unset_n();
        self.f.unset_c();
        self.a &= value;
        if self.a == 0 {
            self.f.set_z();
        } else {
            self.f.unset_z();
        }
        self.cycles += 8;
    }

    make_xor!(xor_a_b, b);
    make_xor!(xor_a_c, c);
    make_xor!(xor_a_d, d);
    make_xor!(xor_a_e, e);
    make_xor!(xor_a_h, h);
    make_xor!(xor_a_l, l);
    #[inline]
    fn xor_a_a(&mut self, _ram: &mut Ram) {
        self.f.unset_h();
        self.f.unset_n();
        self.f.unset_c();
        self.f.set_z();
        self.a = 0;
        self.cycles += 4;
    }

    #[inline]
    fn xor_a_deref_hl(&mut self, ram: &mut Ram) {
        let value = ram[self.hl() as usize];
        self.f.unset_h();
        self.f.unset_n();
        self.f.unset_c();
        self.a ^= value;
        if self.a == 0 {
            self.f.set_z();
        } else {
            self.f.unset_z();
        }
        self.cycles += 8;
    }

    make_or!(or_a_b, b);
    make_or!(or_a_c, c);
    make_or!(or_a_d, d);
    make_or!(or_a_e, e);
    make_or!(or_a_h, h);
    make_or!(or_a_l, l);
    make_or!(or_a_a, a);

    #[inline]
    fn or_a_deref_hl(&mut self, ram: &mut Ram) {
        let value = ram[self.hl() as usize];
        self.f.unset_h();
        self.f.unset_n();
        self.f.unset_c();
        self.a |= value;
        if self.a == 0 {
            self.f.set_z();
        } else {
            self.f.unset_z();
        }
        self.cycles += 8;
    }

    make_cp!(cp_a_b, b);
    make_cp!(cp_a_c, c);
    make_cp!(cp_a_d, d);
    make_cp!(cp_a_e, e);
    make_cp!(cp_a_h, h);
    make_cp!(cp_a_l, l);
    make_cp!(cp_a_a, a);

    #[inline]
    fn cp_a_deref_hl(&mut self, ram: &mut Ram) {
        cp_a_r!(self, ram[self.hl() as usize]);
        self.cycles += 4;
    }

    #[inline]
    fn ret(&mut self, ram: &mut Ram) {
        // TODO: Should popping from empty stack
        // result in a zero or is it an error?
        assert!(
            self.sp.wrapping_add(2) > self.sp,
            "less than 2 bytes of data in the stack"
        );
        self.sp += 1;
        let byte1 = ram[self.sp as usize];
        self.sp += 1;
        let byte2 = ram[self.sp as usize];

        let addr = ((byte2 as u16) << 8) | byte1 as u16;
        self.pc = addr;

        self.cycles += 16;
    }

    make_ret!(ret_nz, z not set);
    make_ret!(ret_z, z set);
    make_ret!(ret_c, c set);
    make_ret!(ret_nc, c not set);

    #[inline]
    fn reti(&mut self, ram: &mut Ram) {
        self.ret(ram);
        self.interrupts = InterruptState::Enabled;
    }

    #[inline]
    fn di(&mut self, _ram: &mut Ram) {
        // Disable interrupts after executing the next instruction
        self.interrupts = InterruptState::WillDisable;
    }

    #[inline]
    fn ei(&mut self, _ram: &mut Ram) {
        // Enable interrupts after executing the next instruction
        self.interrupts = InterruptState::WillEnable;
    }

    make_pop!(pop_bc, b, c);
    make_pop!(pop_de, d, e);
    make_pop!(pop_hl, h, l);

    #[inline]
    fn pop_af(&mut self, ram: &mut Ram) {
        assert!(
            self.sp.wrapping_add(2) > self.sp,
            "less than 2 bytes of data in the stack"
        );
        self.sp += 1;
        let byte = ram[self.sp as usize];
        self.f.0 = byte;
        self.sp += 1;
        let byte = ram[self.sp as usize];
        self.a = byte;

        self.cycles += 12;
    }

    #[inline]
    fn jp(&mut self, ram: &mut Ram) {
        let l = self.next_byte(ram);
        let h = self.next_byte(ram);
        self.pc = ((h as u16) << 8) | l as u16;
        self.cycles += 16;
    }

    make_jp!(jp_nz, z not set);
    make_jp!(jp_z, z set);
    make_jp!(jp_nc, c not set);
    make_jp!(jp_c, c set);

    #[inline]
    fn jp_hl(&mut self, _ram: &mut Ram) {
        self.pc = self.hl();
        self.cycles += 4;
    }

    #[inline]
    fn call(&mut self, ram: &mut Ram) {
        // Address of next instruction is 2 bytes away
        // from this instruction, as this instruction is
        // 3 bytes long but the first byte has been already
        // read when the execution ends up here.
        let addr = self.pc + 2;
        self.push_u16(ram, addr);
        self.jp(ram);
        self.cycles += 8;
    }

    make_call!(call_nz, z not set);
    make_call!(call_z, z set);
    make_call!(call_nc, c not set);
    make_call!(call_c, c set);

    make_push!(push_bc, b, c);
    make_push!(push_de, d, e);
    make_push!(push_hl, h, l);

    #[inline]
    fn push_af(&mut self, ram: &mut Ram) {
        let v1 = self.a;
        let v2 = self.f.0;

        self.push(ram, v1);
        self.push(ram, v2);
        self.cycles += 16;
    }

    make_rst!(rst_00h, 0x0);
    make_rst!(rst_08h, 0x8);
    make_rst!(rst_10h, 0x10);
    make_rst!(rst_18h, 0x18);
    make_rst!(rst_20h, 0x20);
    make_rst!(rst_28h, 0x28);
    make_rst!(rst_30h, 0x30);
    make_rst!(rst_38h, 0x38);

    fn add_a_n(&mut self, ram: &mut Ram) {
        let n = self.next_byte(ram);
        add_a_n!(self, n);
        self.cycles += 4;
    }

    fn adc_a_n(&mut self, ram: &mut Ram) {
        let n = if self.f.c() {
            self.next_byte(ram) + 1
        } else {
            self.next_byte(ram)
        };
        add_a_n!(self, n);
        self.cycles += 4;
    }
}
