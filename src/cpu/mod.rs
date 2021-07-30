#[cfg(test)]
mod tests;

use mmu::Mmu;
use std::default::Default;
use std::fmt;

pub mod cpuflags;
#[macro_use]
mod macros;
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

#[derive(Default, Debug)]
pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: cpuflags::CpuFlags,
    pub pc: u16,
    pub sp: u16,
    run_state: RunState,
    interrupts: InterruptState,
    pub cycles: usize,
    pub debug: bool,
}

impl fmt::Display for Cpu {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\n")?;
        write!(fmt, "a: 0x{:X}, ", self.a)?;
        write!(fmt, "b: 0x{:X}, ", self.b)?;
        write!(fmt, "c: 0x{:X}, ", self.c)?;
        write!(fmt, "d: 0x{:X}, ", self.d)?;
        write!(fmt, "e: 0x{:X}, ", self.e)?;
        write!(fmt, "h: 0x{:X}, ", self.h)?;
        write!(fmt, "l: 0x{:X}\n", self.l)?;
        write!(fmt, "[ ")?;
        if self.f.c() {
            write!(fmt, "c ")?;
        }
        if self.f.h() {
            write!(fmt, "h ")?;
        }
        if self.f.n() {
            write!(fmt, "n ")?;
        }
        if self.f.z() {
            write!(fmt, "z ")?;
        }
        write!(fmt, "] (0x{:X})\n", self.f.0)?;
        write!(fmt, "pc: 0x{:X}\n", self.pc);
        write!(fmt, "sp: 0x{:X}\n", self.sp);
        write!(fmt, "{:?}, ", self.run_state);
        write!(fmt, "interrupts {:?}\n", self.interrupts);
        write!(fmt, "cycles: {}", self.cycles);
        Ok(())
    }
}

impl Cpu {
    #[inline(always)]
    pub fn cycles(&self) -> usize {
        self.cycles
    }
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
    #[inline(always)]
    fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }
    #[inline]
    fn push(&mut self, mmu: &mut Mmu, value: u8) {
        assert!(self.sp > 0);
        mmu.write_u8(self.sp, value);
        self.sp -= 1;
    }
    #[inline]
    fn push_u16(&mut self, mmu: &mut Mmu, value: u16) {
        trace!("PUSHING 0x{:X}", value);
        self.push(mmu, (value >> 8) as u8);
        self.push(mmu, (value & 0xFF) as u8);
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }
    #[inline]
    pub fn reset(&mut self) {
        self.a = 0x01;
        self.f.0 = 0xB0;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.h = 0x01;
        self.l = 0x4D;
        self.sp = 0xFFFE;
        self.pc = 0x100;
        self.interrupts = InterruptState::Enabled; // TODO: This is just a guess
    }
    pub fn step(&mut self, mmu: &mut Mmu) {
        if self.run_state == RunState::Running {
            let opcode = {
                let opcode = self.next_byte(mmu) as usize;
                if opcode == 0xCB {
                    self.next_byte(mmu) as usize + 0x100
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

            info!("{}", opcodes::MNEMONICS[opcode]);
            opcodes::OPCODES[opcode](self, mmu);

            if self.interrupts == old_interrupts_state {
                // The instruction did not modify interrupts flag,
                // enable/disable if we were in WillEnable/WillDisable state
                self.interrupts = new_interrupts_state;
            }
        } else {
            // FIXME: How many cycles should we proceed in time
            // if the CPU is halted?
            self.cycles += 4;
            let intf = mmu.read_u8(0xFF0F);
            if (intf & 0x1) != 0 {
                // V-Blank
                mmu.write_u8(0xFF0F, 0);
                let pc = self.pc;
                self.push_u16(mmu, pc);
                self.pc = 0x0040;
                self.run_state = RunState::Running;
            }
        }
    }

    #[inline]
    fn next_byte(&mut self, mmu: &mut Mmu) -> u8 {
        let ret = mmu.read_u8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        ret
    }

    fn na(&mut self, _: &mut Mmu) {
        panic!("Instruction not available. This is a bug.")
    }

    #[inline]
    fn nop(&mut self, _: &mut Mmu) {
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
    fn add_a_deref_hl(&mut self, mmu: &mut Mmu) {
        add_a_n!(self, mmu.read_u8(self.hl()));
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
    fn inc_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = mmu.read_u8(self.hl());
        mmu.write_u8(self.hl(), inc_r!(self, val));
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
    fn dec_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = mmu.read_u8(self.hl());
        mmu.write_u8(self.hl(), dec_r!(self, val));
        self.cycles += 8;
    }

    make_dec_rr!(dec_bc, b, c);
    make_dec_rr!(dec_de, d, e);
    make_dec_rr!(dec_hl, h, l);
    #[inline]
    fn dec_sp(&mut self, _mmu: &mut Mmu) {
        self.sp = self.sp.wrapping_sub(1);
        self.cycles += 8;
    }
    make_inc_rr!(inc_bc, b, c);
    make_inc_rr!(inc_de, d, e);
    make_inc_rr!(inc_hl, h, l);
    #[inline]
    fn inc_sp(&mut self, _mmu: &mut Mmu) {
        self.sp = self.sp.wrapping_add(1);
        self.cycles += 8;
    }

    make_ld_rr_r!(ld_bc_a, bc, a);
    make_ld_rr_nn!(ld_bc_nn, b, c);
    make_ld_rr_r!(ld_de_a, de, a);
    make_ld_rr_nn!(ld_de_nn, d, e);
    #[inline]
    fn ld_nn_a(&mut self, mmu: &mut Mmu) {
        let mut addr = 0u16;
        addr |= self.next_byte(mmu) as u16;
        addr |= (self.next_byte(mmu) as u16) << 8;
        mmu.write_u8(addr, self.a);
        self.cycles += 16;
    }
    make_ld_rr_nn!(ld_hl_nn, h, l);
    #[inline]
    fn ld_sp_nn(&mut self, mmu: &mut Mmu) {
        self.sp = mmu.read_u16(self.pc);
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
    fn ld_a_nn(&mut self, mmu: &mut Mmu) {
        let mut addr = 0u16;
        addr |= self.next_byte(mmu) as u16;
        addr |= (self.next_byte(mmu) as u16) << 8;
        self.a = mmu.read_u8(addr);
        self.cycles += 16;
    }
    #[inline]
    fn ld_a_addr_c(&mut self, mmu: &mut Mmu) {
        self.a = mmu.read_u8(0xFF00 | self.c as u16);
        self.cycles += 8;
    }
    #[inline]
    fn ld_addr_c_a(&mut self, mmu: &mut Mmu) {
        mmu.write_u8(0xFF00 | self.c as u16, self.a);
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
    fn ld_deref_hl_n(&mut self, mmu: &mut Mmu) {
        let val = self.next_byte(mmu);
        mmu.write_u8(self.hl(), val);
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
    fn rlca(&mut self, _mmu: &mut Mmu) {
        self.a = rlc_n!(self, self.a);
        self.f.unset_z();
        self.cycles += 4;
    }

    #[inline]
    fn rla(&mut self, _mmu: &mut Mmu) {
        self.a = rl_n!(self, self.a);
        self.f.unset_z();
        self.cycles += 4;
    }

    #[inline]
    fn rrca(&mut self, _mmu: &mut Mmu) {
        self.a = rrc_n!(self, self.a);
        self.f.unset_z();
        self.cycles += 4;
    }

    #[inline]
    fn rra(&mut self, _mmu: &mut Mmu) {
        self.a = rr_n!(self, self.a);
        self.f.unset_z();
        self.cycles += 4;
    }

    #[inline]
    fn ld_deref_a16_sp(&mut self, mmu: &mut Mmu) {
        let a16 = mmu.read_u16(self.pc);
        self.pc += 2;
        mmu.write_u16(a16, self.sp);
        self.cycles += 20;
    }

    make_add_rr_rr!(add_hl_bc, h, l, b, c);
    make_add_rr_rr!(add_hl_de, h, l, d, e);
    make_add_rr_rr!(add_hl_hl, h, l, h, l);
    #[inline]
    fn add_hl_sp(&mut self, _mmu: &mut Mmu) {
        let val32_lhs = ((self.h as u32) << 8) & 0xFF00 | self.l as u32 & 0xFF;
        let s = ((self.sp & 0xFF00) >> 8) as u8;
        let p = (self.sp & 0xFF) as u8;
        let mut l = self.l;
        l = l.wrapping_add(p);
        let h_add = if l < self.l { 1 } else { 0 };
        let mut h = self.h;
        h = h.wrapping_add(s).wrapping_add(h_add);
        if cpuflags::test_half_carry_addition(self.h, s.wrapping_add(h_add)) {
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
    fn jr_n(&mut self, mmu: &mut Mmu) {
        let n = self.next_byte(mmu);
        self.pc = self.pc.wrapping_add(n as i8 as u16);
        self.cycles += 12;
    }

    make_jr_cc_n!(jr_nz_n, z not set);
    make_jr_cc_n!(jr_z_n, z set);
    make_jr_cc_n!(jr_nc_n, c not set);
    make_jr_cc_n!(jr_c_n, c set);

    #[inline]
    fn stop(&mut self, _mmu: &mut Mmu) {
        self.run_state = RunState::Stopped;
        self.cycles += 4;
        // TODO: Wake up to a button press
    }

    #[inline]
    fn ld_hli_a(&mut self, mmu: &mut Mmu) {
        mmu.write_u8(self.hl(), self.a);
        let hl = self.hl().wrapping_add(1);
        self.set_hl(hl);
        self.cycles += 8;
    }

    #[inline]
    fn ld_hld_a(&mut self, mmu: &mut Mmu) {
        mmu.write_u8(self.hl(), self.a);
        let hl = self.hl().wrapping_sub(1);
        self.set_hl(hl);
        self.cycles += 8;
    }

    #[inline]
    fn ld_a_hli(&mut self, mmu: &mut Mmu) {
        self.a = mmu.read_u8(self.hl());
        let hl = self.hl().wrapping_add(1);
        self.set_hl(hl);
        self.cycles += 8;
    }

    #[inline]
    fn ld_a_hld(&mut self, mmu: &mut Mmu) {
        self.a = mmu.read_u8(self.hl());
        let hl = self.hl().wrapping_sub(1);
        self.set_hl(hl);
        self.cycles += 8;
    }

    fn daa(&mut self, _mmu: &mut Mmu) {
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
        } else {
            self.f.unset_z();
        }
        self.cycles += 4;
    }

    fn cpl(&mut self, _mmu: &mut Mmu) {
        self.a = !self.a;
        self.f.set_n();
        self.f.set_h();

        self.cycles += 4;
    }

    fn scf(&mut self, _mmu: &mut Mmu) {
        self.f.unset_n();
        self.f.unset_h();
        self.f.set_c();

        self.cycles += 4;
    }

    fn ccf(&mut self, _mmu: &mut Mmu) {
        self.f.unset_n();
        self.f.unset_h();
        if self.f.c() {
            self.f.unset_c();
        } else {
            self.f.set_c();
        }

        self.cycles += 4;
    }

    fn halt(&mut self, _mmu: &mut Mmu) {
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
    fn adc_a_deref_hl(&mut self, mmu: &mut Mmu) {
        let c = if self.f.c() { 1 } else { 0 };
        add_a_n!(self, mmu.read_u8(self.hl()).wrapping_add(c));
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
    fn sub_a_deref_hl(&mut self, mmu: &mut Mmu) {
        sub_a_n!(self, mmu.read_u8(self.hl()));
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
    fn sbc_a_deref_hl(&mut self, mmu: &mut Mmu) {
        let c = if self.f.c() { 1 } else { 0 };
        sub_a_n!(self, mmu.read_u8(self.hl()).wrapping_add(c));
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
    fn and_a_deref_hl(&mut self, mmu: &mut Mmu) {
        let value = mmu.read_u8(self.hl());
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
    fn xor_a_a(&mut self, _mmu: &mut Mmu) {
        self.f.unset_h();
        self.f.unset_n();
        self.f.unset_c();
        self.f.set_z();
        self.a = 0;
        self.cycles += 4;
    }

    #[inline]
    fn xor_a_deref_hl(&mut self, mmu: &mut Mmu) {
        let value = mmu.read_u8(self.hl());
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
    fn or_a_deref_hl(&mut self, mmu: &mut Mmu) {
        let value = mmu.read_u8(self.hl());
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
    fn cp_a_deref_hl(&mut self, mmu: &mut Mmu) {
        cp_a_n!(self, mmu.read_u8(self.hl()));
        self.cycles += 4;
    }

    #[inline]
    fn ret(&mut self, mmu: &mut Mmu) {
        self.sp = self.sp.wrapping_add(1);
        let byte1 = mmu.read_u8(self.sp);
        self.sp = self.sp.wrapping_add(1);
        let byte2 = mmu.read_u8(self.sp);

        let addr = ((byte2 as u16) << 8) | byte1 as u16;
        self.pc = addr;

        self.cycles += 16;
    }

    make_ret!(ret_nz, z not set);
    make_ret!(ret_z, z set);
    make_ret!(ret_c, c set);
    make_ret!(ret_nc, c not set);

    #[inline]
    fn reti(&mut self, mmu: &mut Mmu) {
        self.ret(mmu);
        self.interrupts = InterruptState::Enabled;
    }

    #[inline]
    fn di(&mut self, _mmu: &mut Mmu) {
        // Disable interrupts after executing the next instruction
        self.interrupts = InterruptState::WillDisable;
    }

    #[inline]
    fn ei(&mut self, _mmu: &mut Mmu) {
        // Enable interrupts after executing the next instruction
        self.interrupts = InterruptState::WillEnable;
    }

    make_pop!(pop_bc, b, c);
    make_pop!(pop_de, d, e);
    make_pop!(pop_hl, h, l);

    #[inline]
    fn pop_af(&mut self, mmu: &mut Mmu) {
        assert!(
            self.sp.wrapping_add(2) > self.sp,
            "less than 2 bytes of data in the stack"
        );
        self.sp += 1;
        let byte = mmu.read_u8(self.sp);
        self.f.0 = byte & 0xF0; // Only high 4 bits should be written
        self.sp += 1;
        let byte = mmu.read_u8(self.sp);
        self.a = byte;

        self.cycles += 12;
    }

    #[inline]
    fn jp(&mut self, mmu: &mut Mmu) {
        let l = self.next_byte(mmu);
        let h = self.next_byte(mmu);
        self.pc = ((h as u16) << 8) | l as u16;
        self.cycles += 16;
    }

    make_jp!(jp_nz, z not set);
    make_jp!(jp_z, z set);
    make_jp!(jp_nc, c not set);
    make_jp!(jp_c, c set);

    #[inline]
    fn jp_hl(&mut self, _mmu: &mut Mmu) {
        self.pc = self.hl();
        self.cycles += 4;
    }

    #[inline]
    fn call(&mut self, mmu: &mut Mmu) {
        // Address of next instruction is 2 bytes away
        // from this instruction, as this instruction is
        // 3 bytes long but the first byte has been already
        // read when the execution ends up here.
        let addr = self.pc + 2;
        self.push_u16(mmu, addr);
        self.jp(mmu);
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
    fn push_af(&mut self, mmu: &mut Mmu) {
        let v1 = self.a;
        let v2 = self.f.0;

        self.push(mmu, v1);
        self.push(mmu, v2);
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

    #[inline]
    fn add_a_n(&mut self, mmu: &mut Mmu) {
        let n = self.next_byte(mmu);
        add_a_n!(self, n);
        self.cycles += 4;
    }

    #[inline]
    fn adc_a_n(&mut self, mmu: &mut Mmu) {
        let c = if self.f.c() { 1 } else { 0 };
        let n = self.next_byte(mmu);
        add_a_n_c!(self, n, c);
        self.cycles += 4;
    }

    #[inline]
    fn sub_a_n(&mut self, mmu: &mut Mmu) {
        let n = self.next_byte(mmu);
        sub_a_n!(self, n);
        self.cycles += 4;
    }

    #[inline]
    fn sbc_a_n(&mut self, mmu: &mut Mmu) {
        // TODO: Tests
        let n = self.next_byte(mmu);
        let c = if self.f.c() { 1 } else { 0 };
        sub_a_n_c!(self, n, c);
        self.cycles += 4;
    }

    #[inline]
    fn and_a_n(&mut self, mmu: &mut Mmu) {
        // TODO: Tests
        let n = self.next_byte(mmu);
        and_a_n!(self, n);
        self.cycles += 4;
    }

    #[inline]
    fn xor_a_n(&mut self, mmu: &mut Mmu) {
        // TODO: Tests
        let n = self.next_byte(mmu);
        xor_a_n!(self, n);
        self.cycles += 4;
    }

    #[inline]
    fn or_a_n(&mut self, mmu: &mut Mmu) {
        // TODO: Tests
        let n = self.next_byte(mmu);
        or_a_n!(self, n);
        self.cycles += 4;
    }

    #[inline]
    fn cp_a_n(&mut self, mmu: &mut Mmu) {
        // TODO: Tests
        let n = self.next_byte(mmu);
        cp_a_n!(self, n);
        self.cycles += 4;
    }

    #[inline]
    fn ldh_deref_n_a(&mut self, mmu: &mut Mmu) {
        // TODO: Tests
        let n = self.next_byte(mmu);
        let addr = 0xFF00_u16 + n as u16;
        mmu.write_u8(addr, self.a);
        self.cycles += 12;
    }

    #[inline]
    fn ldh_a_deref_n(&mut self, mmu: &mut Mmu) {
        // TODO: Tests
        let n = self.next_byte(mmu);
        let addr = 0xFF00_u16 + n as u16;
        self.a = mmu.read_u8(addr);
        self.cycles += 12;
    }

    #[inline]
    fn ld_sp_hl(&mut self, _mmu: &mut Mmu) {
        self.sp = self.hl();
    }

    #[inline]
    fn add_sp_n(&mut self, mmu: &mut Mmu) {
        let n = self.next_byte(mmu);
        set_flags_u16_plus_i8!(self, self.sp, n as i8);
        self.sp = self.sp.wrapping_add(n as i8 as u16);
        self.cycles += 16;
    }

    #[inline]
    fn ld_hl_sp_plus_n(&mut self, mmu: &mut Mmu) {
        let n = self.next_byte(mmu);
        set_flags_u16_plus_i8!(self, self.sp, n as i8);
        let hl = self.sp.wrapping_add(n as i8 as u16);
        self.set_hl(hl);
        self.cycles += 12;
    }

    make_rlc_r!(rlc_b, b);
    make_rlc_r!(rlc_c, c);
    make_rlc_r!(rlc_d, d);
    make_rlc_r!(rlc_e, e);
    make_rlc_r!(rlc_h, h);
    make_rlc_r!(rlc_l, l);
    make_rlc_r!(rlc_a, a);
    #[inline]
    fn rlc_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = rlc_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_rrc_r!(rrc_b, b);
    make_rrc_r!(rrc_c, c);
    make_rrc_r!(rrc_d, d);
    make_rrc_r!(rrc_e, e);
    make_rrc_r!(rrc_h, h);
    make_rrc_r!(rrc_l, l);
    make_rrc_r!(rrc_a, a);
    #[inline]
    fn rrc_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = rrc_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_rl_r!(rl_b, b);
    make_rl_r!(rl_c, c);
    make_rl_r!(rl_d, d);
    make_rl_r!(rl_e, e);
    make_rl_r!(rl_h, h);
    make_rl_r!(rl_l, l);
    make_rl_r!(rl_a, a);
    #[inline]
    fn rl_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = rl_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_rr_r!(rr_b, b);
    make_rr_r!(rr_c, c);
    make_rr_r!(rr_d, d);
    make_rr_r!(rr_e, e);
    make_rr_r!(rr_h, h);
    make_rr_r!(rr_l, l);
    make_rr_r!(rr_a, a);
    #[inline]
    fn rr_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = rr_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_sla!(sla_b, b);
    make_sla!(sla_c, c);
    make_sla!(sla_d, d);
    make_sla!(sla_e, e);
    make_sla!(sla_h, h);
    make_sla!(sla_l, l);
    make_sla!(sla_a, a);
    #[inline]
    fn sla_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = sla_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_sra!(sra_b, b);
    make_sra!(sra_c, c);
    make_sra!(sra_d, d);
    make_sra!(sra_e, e);
    make_sra!(sra_h, h);
    make_sra!(sra_l, l);
    make_sra!(sra_a, a);
    #[inline]
    fn sra_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = sra_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_swap!(swap_b, b);
    make_swap!(swap_c, c);
    make_swap!(swap_d, d);
    make_swap!(swap_e, e);
    make_swap!(swap_h, h);
    make_swap!(swap_l, l);
    make_swap!(swap_a, a);
    #[inline]
    fn swap_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = swap_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_srl!(srl_b, b);
    make_srl!(srl_c, c);
    make_srl!(srl_d, d);
    make_srl!(srl_e, e);
    make_srl!(srl_h, h);
    make_srl!(srl_l, l);
    make_srl!(srl_a, a);
    #[inline]
    fn srl_deref_hl(&mut self, mmu: &mut Mmu) {
        let val = srl_n!(self, mmu.read_u8(self.hl()));
        self.set_hl(val as u16);
        self.cycles += 16;
    }

    make_bit!(bit0_b, 0, b);
    make_bit!(bit0_c, 0, c);
    make_bit!(bit0_d, 0, d);
    make_bit!(bit0_e, 0, e);
    make_bit!(bit0_h, 0, h);
    make_bit!(bit0_l, 0, l);
    make_bit_deref_hl!(bit0_deref_hl, 0);
    make_bit!(bit0_a, 0, a);

    make_bit!(bit1_b, 1, b);
    make_bit!(bit1_c, 1, c);
    make_bit!(bit1_d, 1, d);
    make_bit!(bit1_e, 1, e);
    make_bit!(bit1_h, 1, h);
    make_bit!(bit1_l, 1, l);
    make_bit_deref_hl!(bit1_deref_hl, 1);
    make_bit!(bit1_a, 1, a);

    make_bit!(bit2_b, 2, b);
    make_bit!(bit2_c, 2, c);
    make_bit!(bit2_d, 2, d);
    make_bit!(bit2_e, 2, e);
    make_bit!(bit2_h, 2, h);
    make_bit!(bit2_l, 2, l);
    make_bit_deref_hl!(bit2_deref_hl, 2);
    make_bit!(bit2_a, 2, a);

    make_bit!(bit3_b, 3, b);
    make_bit!(bit3_c, 3, c);
    make_bit!(bit3_d, 3, d);
    make_bit!(bit3_e, 3, e);
    make_bit!(bit3_h, 3, h);
    make_bit!(bit3_l, 3, l);
    make_bit_deref_hl!(bit3_deref_hl, 3);
    make_bit!(bit3_a, 3, a);

    make_bit!(bit4_b, 4, b);
    make_bit!(bit4_c, 4, c);
    make_bit!(bit4_d, 4, d);
    make_bit!(bit4_e, 4, e);
    make_bit!(bit4_h, 4, h);
    make_bit!(bit4_l, 4, l);
    make_bit_deref_hl!(bit4_deref_hl, 4);
    make_bit!(bit4_a, 4, a);

    make_bit!(bit5_b, 5, b);
    make_bit!(bit5_c, 5, c);
    make_bit!(bit5_d, 5, d);
    make_bit!(bit5_e, 5, e);
    make_bit!(bit5_h, 5, h);
    make_bit!(bit5_l, 5, l);
    make_bit_deref_hl!(bit5_deref_hl, 5);
    make_bit!(bit5_a, 5, a);

    make_bit!(bit6_b, 6, b);
    make_bit!(bit6_c, 6, c);
    make_bit!(bit6_d, 6, d);
    make_bit!(bit6_e, 6, e);
    make_bit!(bit6_h, 6, h);
    make_bit!(bit6_l, 6, l);
    make_bit_deref_hl!(bit6_deref_hl, 6);
    make_bit!(bit6_a, 6, a);

    make_bit!(bit7_b, 7, b);
    make_bit!(bit7_c, 7, c);
    make_bit!(bit7_d, 7, d);
    make_bit!(bit7_e, 7, e);
    make_bit!(bit7_h, 7, h);
    make_bit!(bit7_l, 7, l);
    make_bit_deref_hl!(bit7_deref_hl, 7);
    make_bit!(bit7_a, 7, a);

    make_res!(res0_b, 0, b);
    make_res!(res0_c, 0, c);
    make_res!(res0_d, 0, d);
    make_res!(res0_e, 0, e);
    make_res!(res0_h, 0, h);
    make_res!(res0_l, 0, l);
    make_res_deref_hl!(res0_deref_hl, 0);
    make_res!(res0_a, 0, a);

    make_res!(res1_b, 1, b);
    make_res!(res1_c, 1, c);
    make_res!(res1_d, 1, d);
    make_res!(res1_e, 1, e);
    make_res!(res1_h, 1, h);
    make_res!(res1_l, 1, l);
    make_res_deref_hl!(res1_deref_hl, 1);
    make_res!(res1_a, 1, a);

    make_res!(res2_b, 2, b);
    make_res!(res2_c, 2, c);
    make_res!(res2_d, 2, d);
    make_res!(res2_e, 2, e);
    make_res!(res2_h, 2, h);
    make_res!(res2_l, 2, l);
    make_res_deref_hl!(res2_deref_hl, 2);
    make_res!(res2_a, 2, a);

    make_res!(res3_b, 3, b);
    make_res!(res3_c, 3, c);
    make_res!(res3_d, 3, d);
    make_res!(res3_e, 3, e);
    make_res!(res3_h, 3, h);
    make_res!(res3_l, 3, l);
    make_res_deref_hl!(res3_deref_hl, 3);
    make_res!(res3_a, 3, a);

    make_res!(res4_b, 4, b);
    make_res!(res4_c, 4, c);
    make_res!(res4_d, 4, d);
    make_res!(res4_e, 4, e);
    make_res!(res4_h, 4, h);
    make_res!(res4_l, 4, l);
    make_res_deref_hl!(res4_deref_hl, 4);
    make_res!(res4_a, 4, a);

    make_res!(res5_b, 5, b);
    make_res!(res5_c, 5, c);
    make_res!(res5_d, 5, d);
    make_res!(res5_e, 5, e);
    make_res!(res5_h, 5, h);
    make_res!(res5_l, 5, l);
    make_res_deref_hl!(res5_deref_hl, 5);
    make_res!(res5_a, 5, a);

    make_res!(res6_b, 6, b);
    make_res!(res6_c, 6, c);
    make_res!(res6_d, 6, d);
    make_res!(res6_e, 6, e);
    make_res!(res6_h, 6, h);
    make_res!(res6_l, 6, l);
    make_res_deref_hl!(res6_deref_hl, 6);
    make_res!(res6_a, 6, a);

    make_res!(res7_b, 7, b);
    make_res!(res7_c, 7, c);
    make_res!(res7_d, 7, d);
    make_res!(res7_e, 7, e);
    make_res!(res7_h, 7, h);
    make_res!(res7_l, 7, l);
    make_res_deref_hl!(res7_deref_hl, 7);
    make_res!(res7_a, 7, a);

    make_set!(set0_b, 0, b);
    make_set!(set0_c, 0, c);
    make_set!(set0_d, 0, d);
    make_set!(set0_e, 0, e);
    make_set!(set0_h, 0, h);
    make_set!(set0_l, 0, l);
    make_set_deref_hl!(set0_deref_hl, 0);
    make_set!(set0_a, 0, a);

    make_set!(set1_b, 1, b);
    make_set!(set1_c, 1, c);
    make_set!(set1_d, 1, d);
    make_set!(set1_e, 1, e);
    make_set!(set1_h, 1, h);
    make_set!(set1_l, 1, l);
    make_set_deref_hl!(set1_deref_hl, 1);
    make_set!(set1_a, 1, a);

    make_set!(set2_b, 2, b);
    make_set!(set2_c, 2, c);
    make_set!(set2_d, 2, d);
    make_set!(set2_e, 2, e);
    make_set!(set2_h, 2, h);
    make_set!(set2_l, 2, l);
    make_set_deref_hl!(set2_deref_hl, 2);
    make_set!(set2_a, 2, a);

    make_set!(set3_b, 3, b);
    make_set!(set3_c, 3, c);
    make_set!(set3_d, 3, d);
    make_set!(set3_e, 3, e);
    make_set!(set3_h, 3, h);
    make_set!(set3_l, 3, l);
    make_set_deref_hl!(set3_deref_hl, 3);
    make_set!(set3_a, 3, a);

    make_set!(set4_b, 4, b);
    make_set!(set4_c, 4, c);
    make_set!(set4_d, 4, d);
    make_set!(set4_e, 4, e);
    make_set!(set4_h, 4, h);
    make_set!(set4_l, 4, l);
    make_set_deref_hl!(set4_deref_hl, 4);
    make_set!(set4_a, 4, a);

    make_set!(set5_b, 5, b);
    make_set!(set5_c, 5, c);
    make_set!(set5_d, 5, d);
    make_set!(set5_e, 5, e);
    make_set!(set5_h, 5, h);
    make_set!(set5_l, 5, l);
    make_set_deref_hl!(set5_deref_hl, 5);
    make_set!(set5_a, 5, a);

    make_set!(set6_b, 6, b);
    make_set!(set6_c, 6, c);
    make_set!(set6_d, 6, d);
    make_set!(set6_e, 6, e);
    make_set!(set6_h, 6, h);
    make_set!(set6_l, 6, l);
    make_set_deref_hl!(set6_deref_hl, 6);
    make_set!(set6_a, 6, a);

    make_set!(set7_b, 7, b);
    make_set!(set7_c, 7, c);
    make_set!(set7_d, 7, d);
    make_set!(set7_e, 7, e);
    make_set!(set7_h, 7, h);
    make_set!(set7_l, 7, l);
    make_set_deref_hl!(set7_deref_hl, 7);
    make_set!(set7_a, 7, a);
}
