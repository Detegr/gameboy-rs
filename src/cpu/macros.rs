macro_rules! make_ld_rr_nn {
    ($name: ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            self.$r2 = self.next_byte(mmu);
            self.$r1 = self.next_byte(mmu);
            self.cycles += 12;
        }
    }
}
macro_rules! make_ld_r_r {
    ($name:ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r1 = self.$r2;
            self.cycles += 4;
        }
    }
}
macro_rules! make_ld_r_rr {
    ($name: ident, $r:ident, $rr:ident) => {
        fn $name(&mut self, mmu: &mut Mmu) {
            self.$r = mmu.read_u8(self.$rr());
            self.cycles += 8;
        }
    }
}
macro_rules! make_ld_rr_r {
    ($name: ident, $rr:ident, $r:ident) => {
        fn $name(&mut self, mmu: &mut Mmu) {
            mmu.write_u8(self.$rr(), self.$r);
            self.cycles += 8;
        }
    }
}
macro_rules! make_ld_r_n {
    ($name: ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            self.$r = self.next_byte(mmu);
            self.cycles += 8;
        }
    }
}
macro_rules! make_add {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            add_a_n!(self, self.$reg);
        }
    }
}
macro_rules! make_adc {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            add_a_n_c!(self, self.$reg, if self.f.c() { 1 } else { 0 });
        }
    }
}
macro_rules! make_add_rr_rr {
    ($name:ident, $r1: ident, $r2:ident, $r3:ident, $r4:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            let val32_lhs = ((self.$r1 as u32) << 8) & 0xFF00 | self.$r2 as u32 & 0xFF;
            let val32_rhs = ((self.$r3 as u32) << 8) & 0xFF00 | self.$r4 as u32 & 0xFF;
            let mut r2 = self.$r2;
            r2 = r2.wrapping_add(self.$r4);
            let r1_add = if r2 < self.$r2 { 1 } else { 0 };
            let mut r1 = self.$r1;
            r1 = r1.wrapping_add(self.$r3).wrapping_add(r1_add);
            if cpuflags::test_half_carry_addition(self.$r1, self.$r3.wrapping_add(r1_add)) {
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
macro_rules! add_a_n_c {
    ($cpu:expr, $value:expr, $carry:expr) => {
        $cpu.f.unset_n();
        let val = $value;
        let carry = $carry;
        let check = ($cpu.a as u16) + (val as u16) + (carry as u16);
        if check > 0xFF {
            $cpu.f.set_h();
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_h();
            $cpu.f.unset_c();
        }
        let check_without_carry_bit = ($cpu.a as u16) ^ (val as u16) ^ (carry as u16);
        if ((check ^ check_without_carry_bit) & 0x10) != 0 {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        $cpu.a = $cpu.a.wrapping_add(val).wrapping_add(carry);
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    };
}
macro_rules! add_a_n {
    ($cpu:expr, $value:expr) => {
        $cpu.f.unset_n();
        let mut val = $value;
        let check = ($cpu.a as u16) + (val as u16);
        if check > 0xFF {
            $cpu.f.set_h();
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_h();
            $cpu.f.unset_c();
        }
        if cpuflags::test_half_carry_addition($cpu.a, val) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        $cpu.a = $cpu.a.wrapping_add(val);
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    };
}
macro_rules! make_sub {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            sub_a_n!(self, self.$reg);
        }
    }
}
macro_rules! make_sbc {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            if self.f.c() {
                sub_a_n_c!(self, self.$reg, 1);
            } else {
                sub_a_n_c!(self, self.$reg, 0);
            }
        }
    }
}
macro_rules! sub_a_n_c {
    ($cpu:expr, $value:expr, $carry:expr) => {
        let val = $value;
        let carry = $carry;
        let check = ($cpu.a as u16)
            .wrapping_sub(val as u16)
            .wrapping_sub(carry as u16);
        if check & 0x100 != 0 {
            $cpu.f.set_h();
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_h();
            $cpu.f.unset_c();
        }
        let check_without_carry_bit = ($cpu.a as u16) ^ (val as u16) ^ (carry as u16);
        if ((check ^ check_without_carry_bit) & 0x10) != 0 {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        $cpu.a = $cpu.a.wrapping_sub(val).wrapping_sub(carry);
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.f.set_n();
        $cpu.cycles += 4;
    };
}
macro_rules! sub_a_n {
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
        fn $name(&mut self, _mmu: &mut Mmu) {
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
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$reg = inc_r!(self, self.$reg);
        }
    }
}
macro_rules! inc_r {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;
        if cpuflags::test_half_carry_addition(val, 1) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        val = val.wrapping_add(1);
        if val == 0x0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.f.unset_n();
        $cpu.cycles += 4;
        val
    }};
}
macro_rules! make_dec {
    ($name:ident, $reg: ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$reg = dec_r!(self, self.$reg);
        }
    }
}
macro_rules! dec_r {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;
        if cpuflags::test_half_carry_subtraction(val, 1) {
            $cpu.f.set_h();
        } else {
            $cpu.f.unset_h();
        }
        val = val.wrapping_sub(1);
        if val == 0x0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.f.set_n();
        $cpu.cycles += 4;
        val
    }};
}
macro_rules! make_dec_rr {
    ($name:ident, $r1:ident, $r2:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
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
        fn $name(&mut self, mmu: &mut Mmu) {
            if self.f.$flag() {
                self.jr_n(mmu);
            } else {
                self.pc += 1;
                self.cycles += 8;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            if !self.f.$flag() {
                self.jr_n(mmu);
            } else {
                self.pc += 1;
                self.cycles += 8;
            }
        }
    }
}
macro_rules! and_a_n {
    ($cpu:expr, $value:expr) => {
        $cpu.f.set_h();
        $cpu.f.unset_n();
        $cpu.f.unset_c();
        $cpu.a &= $value;
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    };
}
macro_rules! make_and {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            and_a_n!(self, self.$r);
        }
    }
}
macro_rules! or_a_n {
    ($cpu:expr, $value:expr) => {
        $cpu.f.unset_h();
        $cpu.f.unset_n();
        $cpu.f.unset_c();
        $cpu.a |= $value;
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    };
}
macro_rules! make_or {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            or_a_n!(self, self.$r);
        }
    }
}
macro_rules! xor_a_n {
    ($cpu:expr, $value:expr) => {
        $cpu.f.unset_h();
        $cpu.f.unset_n();
        $cpu.f.unset_c();
        $cpu.a ^= $value;
        if $cpu.a == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        $cpu.cycles += 4;
    };
}
macro_rules! make_xor {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            xor_a_n!(self, self.$r);
        }
    }
}
macro_rules! cp_a_n {
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
        fn $name(&mut self, _mmu: &mut Mmu) {
            cp_a_n!(self, self.$r)
        }
    }
}
macro_rules! make_ret {
    ($name:ident, $flag:ident set) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            if self.f.$flag() {
                self.ret(mmu);
                self.cycles += 4;
            } else {
                self.cycles += 8;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            if !self.f.$flag() {
                self.ret(mmu);
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
        fn $name(&mut self, mmu: &mut Mmu) {
            assert!(
                self.sp.wrapping_add(2) > self.sp,
                "less than 2 bytes of data in the stack"
            );
            self.sp += 1;
            let byte = mmu.read_u8(self.sp);
            self.$r2 = byte;
            self.sp += 1;
            let byte = mmu.read_u8(self.sp);
            self.$r1 = byte;

            self.cycles += 12;
        }
    }
}
macro_rules! make_jp {
    ($name:ident, $flag:ident set) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            if self.f.$flag() {
                self.jp(mmu);
            } else {
                self.cycles += 12;
                self.pc += 2;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            if !self.f.$flag() {
                self.jp(mmu);
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
        fn $name(&mut self, mmu: &mut Mmu) {
            if self.f.$flag() {
                self.call(mmu);
            } else {
                self.pc += 2;
                self.cycles += 12;
            }
        }
    };
    ($name:ident, $flag:ident not set) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            if !self.f.$flag() {
                self.call(mmu);
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
        fn $name(&mut self, mmu: &mut Mmu) {
            assert!(
                self.sp.wrapping_sub(2) < self.sp,
                "stack overflow"
            );

            let v1 = self.$r1;
            let v2 = self.$r2;

            self.push(mmu, v1);
            self.push(mmu, v2);

            self.cycles += 16;
        }
    }
}
macro_rules! make_rst {
    ($name:ident, $to:expr) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            let val = self.pc;
            self.push_u16(mmu, val);
            self.pc = $to;
            self.cycles += 16;
        }
    }
}
macro_rules! set_flags_u16_plus_i8 {
    ($cpu:expr, $u16:expr, $i8:expr) => {
        $cpu.f.unset_n();
        $cpu.f.unset_z();
        // Half carry and carry flags are calculated
        // with lower 8 bits
        let check = ($u16 & 0xFF) as i16 + ($i8 as i16);
        if check > 0xFF || check < 0 {
            $cpu.f.set_h();
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_h();
            $cpu.f.unset_c();
        }
        if $i8 > 0 {
            if cpuflags::test_half_carry_addition(($u16 & 0xFF) as u8, $i8 as u8) {
                $cpu.f.set_h();
            } else {
                $cpu.f.unset_h();
            }
        } else {
            if cpuflags::test_half_carry_subtraction(($u16 & 0xFF) as u8, (-$i8) as u8) {
                $cpu.f.set_h();
            } else {
                $cpu.f.unset_h();
            }
        }
    };
}
macro_rules! rlc_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;

        $cpu.f.unset_n();
        $cpu.f.unset_h();
        if (val & 0x80) != 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }

        val = val.rotate_left(1);

        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }

        val
    }};
}
macro_rules! make_rlc_r {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = rlc_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}

macro_rules! rrc_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;

        $cpu.f.unset_n();
        $cpu.f.unset_h();
        if (val & 0x1) != 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }

        val = val.rotate_right(1);

        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }

        val
    }};
}
macro_rules! make_rrc_r {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = rrc_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}

macro_rules! rl_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;

        $cpu.f.unset_n();
        $cpu.f.unset_h();
        let old_carry = $cpu.f.c();
        if (val & 0x80) != 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }
        val <<= 1;
        if old_carry {
            val |= 0x1;
        }
        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        val
    }};
}
macro_rules! make_rl_r {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = rl_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}
macro_rules! rr_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;

        $cpu.f.unset_n();
        $cpu.f.unset_h();
        let old_carry = $cpu.f.c();
        if (val & 0x1) != 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }
        val >>= 1;
        if old_carry {
            val |= 0x80;
        }
        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        val
    }};
}
macro_rules! make_rr_r {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = rr_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}
macro_rules! sla_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;

        $cpu.f.unset_n();
        $cpu.f.unset_h();
        if (val & 0x80) != 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }
        val <<= 1;
        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        val
    }};
}
macro_rules! make_sla {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = sla_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}
macro_rules! sra_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;
        $cpu.f.unset_n();
        $cpu.f.unset_h();
        if (val & 0x1) != 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }
        let bit7 = (val & 0x80) != 0;
        val >>= 1;
        if bit7 {
            val |= 0x80;
        }
        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        val
    }};
}
macro_rules! make_sra {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = sra_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}

macro_rules! swap_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;

        $cpu.f.unset_c();
        $cpu.f.unset_n();
        $cpu.f.unset_h();

        let upper = val & 0xF0;
        let lower = val & 0xF;

        val = (lower << 4) | (upper >> 4);

        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }

        val
    }};
}
macro_rules! make_swap {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = swap_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}
macro_rules! srl_n {
    ($cpu:expr, $value:expr) => {{
        let mut val = $value;

        $cpu.f.unset_n();
        $cpu.f.unset_h();
        if (val & 0x1) != 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }
        val >>= 1;
        if val == 0 {
            $cpu.f.set_z();
        } else {
            $cpu.f.unset_z();
        }
        val
    }};
}
macro_rules! make_srl {
    ($name:ident, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = srl_n!(self, self.$r);
            self.cycles += 8;
        }
    }
}

macro_rules! bit_n_n {
    ($cpu:expr, $bit:expr, $value: expr) => {
        $cpu.f.unset_n();
        $cpu.f.set_h();
        let mask = 0x1u8 << $bit;
        if ($value & mask) == 0 {
            $cpu.f.set_c();
        } else {
            $cpu.f.unset_c();
        }
    };
}

macro_rules! make_bit {
    ($name:ident, $bit:expr, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            bit_n_n!(self, $bit, self.$r);
            self.cycles += 8;
        }
    }
}

macro_rules! make_bit_deref_hl {
    ($name:ident, $bit:expr) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            let val = sra_n!(self, mmu.read_u8(self.hl()));
            bit_n_n!(self, $bit, val);
            self.cycles += 16;
        }
    }
}

macro_rules! res_n_n {
    ($cpu:expr, $bit:expr, $value: expr) => {{
        let mut val = $value;
        let mask = 0x1u8 << $bit;
        val &= !mask;
        val
    }};
}

macro_rules! make_res {
    ($name:ident, $bit:expr, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = res_n_n!(self, $bit, self.$r);
            self.cycles += 8;
        }
    }
}

macro_rules! make_res_deref_hl {
    ($name:ident, $bit:expr) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            let val = res_n_n!(self, $bit, mmu.read_u8(self.hl()));
            mmu.write_u8(self.hl(), val);
            self.cycles += 16;
        }
    }
}

macro_rules! set_n_n {
    ($cpu:expr, $bit:expr, $value: expr) => {{
        let mut val = $value;
        let mask = 0x1u8 << $bit;
        val |= mask;
        val
    }};
}
macro_rules! make_set {
    ($name:ident, $bit:expr, $r:ident) => {
        #[inline]
        fn $name(&mut self, _mmu: &mut Mmu) {
            self.$r = set_n_n!(self, $bit, self.$r);
            self.cycles += 8;
        }
    }
}
macro_rules! make_set_deref_hl {
    ($name:ident, $bit:expr) => {
        #[inline]
        fn $name(&mut self, mmu: &mut Mmu) {
            let val = set_n_n!(self, $bit, mmu.read_u8(self.hl()));
            mmu.write_u8(self.hl(), val);
            self.cycles += 16;
        }
    }
}
