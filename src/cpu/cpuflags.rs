#[derive(Default)]
pub struct CpuFlags(u8);
impl CpuFlags {
    #[inline(always)]
    fn z(&self) -> bool {
        (self.0 & 0x80) != 0
    }
    #[inline(always)]
    fn set_z(&mut self) {
        self.0 |= 0x80;
    }
    #[inline(always)]
    fn unset_z(&mut self) {
        self.0 &= !0x80;
    }

    #[inline(always)]
    fn n(&self) -> bool {
        (self.0 & 0x40) != 0
    }
    #[inline(always)]
    fn set_n(&mut self) {
        self.0 |= 0x40
    }
    #[inline(always)]
    fn unset_n(&mut self) {
        self.0 &= !0x40;
    }

    #[inline(always)]
    fn h(&self) -> bool {
        (self.0 & 0x20) != 0
    }
    #[inline(always)]
    fn set_h(&mut self) {
        self.0 |= 0x20
    }
    #[inline(always)]
    fn unset_h(&mut self) {
        self.0 &= !0x20;
    }

    #[inline(always)]
    fn c(&self) -> bool {
        (self.0 & 0x10) != 0
    }
    #[inline(always)]
    fn set_c(&mut self) {
        self.0 |= 0x10
    }
    #[inline(always)]
    fn unset_c(&mut self) {
        self.0 &= !0x10;
    }
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
