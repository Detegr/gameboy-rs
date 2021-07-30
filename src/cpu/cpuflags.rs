use std::fmt;

#[derive(Default, Debug)]
pub struct CpuFlags(pub u8);
impl CpuFlags {
    #[inline(always)]
    pub fn z(&self) -> bool {
        (self.0 & 0x80) != 0
    }
    #[inline(always)]
    pub fn set_z(&mut self) {
        self.0 |= 0x80;
    }
    #[inline(always)]
    pub fn unset_z(&mut self) {
        self.0 &= !0x80;
    }

    #[inline(always)]
    pub fn n(&self) -> bool {
        (self.0 & 0x40) != 0
    }
    #[inline(always)]
    pub fn set_n(&mut self) {
        self.0 |= 0x40
    }
    #[inline(always)]
    pub fn unset_n(&mut self) {
        self.0 &= !0x40;
    }

    #[inline(always)]
    pub fn h(&self) -> bool {
        (self.0 & 0x20) != 0
    }
    #[inline(always)]
    pub fn set_h(&mut self) {
        self.0 |= 0x20
    }
    #[inline(always)]
    pub fn unset_h(&mut self) {
        self.0 &= !0x20;
    }

    #[inline(always)]
    pub fn c(&self) -> bool {
        (self.0 & 0x10) != 0
    }
    #[inline(always)]
    pub fn set_c(&mut self) {
        self.0 |= 0x10
    }
    #[inline(always)]
    pub fn unset_c(&mut self) {
        self.0 &= !0x10;
    }
}

impl fmt::Display for CpuFlags {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{} {} {} {}",
            if self.z() { "Z" } else { "" },
            if self.n() { "N" } else { "" },
            if self.h() { "H" } else { "" },
            if self.c() { "C" } else { "" }
        )
    }
}

#[inline(always)]
pub fn test_half_carry_addition(a: u8, b: u8) -> bool {
    ((a & 0xF).wrapping_add(b & 0xF) & 0x10) == 0x10
}

#[inline(always)]
pub fn test_half_carry_addition_u16(a: u16, b: u16) -> bool {
    ((a & 0xF).wrapping_add(b & 0xF) & 0x10) == 0x10
}

#[inline(always)]
pub fn test_half_carry_subtraction(a: u8, b: u8) -> bool {
    ((a & 0xF).wrapping_sub(b & 0xF) & 0x10) == 0x10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z() {
        let mut f = CpuFlags::default();
        assert!(!f.z());
        assert!(!f.n());
        assert!(!f.h());
        assert!(!f.c());
        assert!(f.0 == 0b00000000);
        f.set_z();
        assert!(f.z());
        assert!(!f.n());
        assert!(!f.h());
        assert!(!f.c());
        assert!(f.0 == 0b10000000);
        f.unset_z();
        assert!(!f.z());
        assert!(!f.n());
        assert!(!f.h());
        assert!(!f.c());
        assert!(f.0 == 0b00000000);
    }
    #[test]
    fn test_n() {
        let mut f = CpuFlags::default();
        f.set_n();
        assert!(!f.z());
        assert!(f.n());
        assert!(!f.h());
        assert!(!f.c());
        assert!(f.0 == 0b01000000);
        f.unset_n();
        assert!(!f.z());
        assert!(!f.n());
        assert!(!f.h());
        assert!(!f.c());
        assert!(f.0 == 0b00000000);
    }
    #[test]
    fn test_h() {
        let mut f = CpuFlags::default();
        f.set_h();
        assert!(!f.z());
        assert!(!f.n());
        assert!(f.h());
        assert!(!f.c());
        assert!(f.0 == 0b00100000);
        f.unset_h();
        assert!(!f.z());
        assert!(!f.n());
        assert!(!f.h());
        assert!(!f.c());
        assert!(f.0 == 0b00000000);
    }
    #[test]
    fn test_c() {
        let mut f = CpuFlags::default();
        f.set_c();
        assert!(!f.z());
        assert!(!f.n());
        assert!(!f.h());
        assert!(f.c());
        assert!(f.0 == 0b00010000);
        f.unset_c();
        assert!(!f.z());
        assert!(!f.n());
        assert!(!f.h());
        assert!(!f.c());
        assert!(f.0 == 0b00000000);
    }
}
