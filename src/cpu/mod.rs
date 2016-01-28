use byteorder::{ByteOrder, LittleEndian};
use std::default::Default;
use mmu::Mmu;

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
    f: u8,
    pc: usize, // Actually u16 but defined as usize to avoid casting for indexing
    sp: u16,
    cycles: usize,
}
impl Cpu {
    fn hl(&self) -> u16 {
        let mut ret = 0u16;
        ret |= (self.h as u16) << 8;
        ret |= self.l as u16;
        ret
    }
}
macro_rules! ld_n_nn {
    ($cpu:expr, $mmu:expr, $n1:ident, $n2:ident) => {
        $cpu.$n2=$cpu.next_byte($mmu);
        $cpu.$n1=$cpu.next_byte($mmu);
        $cpu.cycles += 12;
    }
}
macro_rules! ld_r1_r2 {
    ($cpu:expr, $r1:ident, $r2:ident) => {
        $cpu.$r1 = $cpu.$r2
        $cpu.cycles += 4;
    }
}
macro_rules! ld_r_hl {
    ($cpu:expr, $mmu:expr, $r:ident) => {
        $cpu.$r = $mmu[$cpu.hl() as usize];
        $cpu.cycles += 8;
    }
}
macro_rules! ld_hl_r {
    ($cpu:expr, $mmu:expr, $r:ident) => {
        $mmu[$cpu.hl() as usize] = $cpu.$r;
        $cpu.cycles += 8;
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu::default()
    }
    pub fn _reset(&mut self) {
        self.sp = 0xFFFE;
        self.pc = 0x100;
    }
    fn next_byte(&mut self, mmu: &mut Mmu) -> u8 {
        if self.pc > 65536 {
            panic!("Program counter out of bounds")
        }
        let ret = mmu[self.pc];
        self.pc += 1;
        ret
    }
    fn nyi(&mut self, _: &mut Mmu) {
        panic!("Instruction not yet implemented")
    }
    fn nop(&mut self, _: &mut Mmu) {
        self.cycles += 8;
    }

    fn ld_bc_nn(&mut self, mmu: &mut Mmu) { ld_n_nn!(self, mmu, b, c); }
    fn ld_de_nn(&mut self, mmu: &mut Mmu) { ld_n_nn!(self, mmu, d, e); }
    fn ld_hl_nn(&mut self, mmu: &mut Mmu) { ld_n_nn!(self, mmu, h, l); }
    fn ld_sp_nn(&mut self, mmu: &mut Mmu) {
        self.sp = LittleEndian::read_u16(&mmu[self.pc..]);
        self.pc += 2;
        self.cycles += 12;
    }

    fn ld_a_a(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, a, a); }
    fn ld_a_b(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, a, b); }
    fn ld_a_c(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, a, c); }
    fn ld_a_d(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, a, d); }
    fn ld_a_e(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, a, e); }
    fn ld_a_h(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, a, h); }
    fn ld_a_l(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, a, l); }
    fn ld_a_hl(&mut self, mmu: &mut Mmu) { ld_r_hl!(self, mmu, a); }

    fn ld_b_a(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, b, a); }
    fn ld_b_b(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, b, b); }
    fn ld_b_c(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, b, c); }
    fn ld_b_d(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, b, d); }
    fn ld_b_e(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, b, e); }
    fn ld_b_h(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, b, h); }
    fn ld_b_l(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, b, l); }
    fn ld_b_hl(&mut self, mmu: &mut Mmu) { ld_r_hl!(self, mmu, b); }

    fn ld_c_a(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, c, a); }
    fn ld_c_b(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, c, b); }
    fn ld_c_c(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, c, c); }
    fn ld_c_d(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, c, d); }
    fn ld_c_e(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, c, e); }
    fn ld_c_h(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, c, h); }
    fn ld_c_l(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, c, l); }
    fn ld_c_hl(&mut self, mmu: &mut Mmu) { ld_r_hl!(self, mmu, c); }

    fn ld_d_a(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, d, a); }
    fn ld_d_b(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, d, b); }
    fn ld_d_c(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, d, c); }
    fn ld_d_d(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, d, d); }
    fn ld_d_e(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, d, e); }
    fn ld_d_h(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, d, h); }
    fn ld_d_l(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, d, l); }
    fn ld_d_hl(&mut self, mmu: &mut Mmu) { ld_r_hl!(self, mmu, d); }

    fn ld_e_a(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, e, a); }
    fn ld_e_b(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, e, b); }
    fn ld_e_c(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, e, c); }
    fn ld_e_d(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, e, d); }
    fn ld_e_e(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, e, e); }
    fn ld_e_h(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, e, h); }
    fn ld_e_l(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, e, l); }
    fn ld_e_hl(&mut self, mmu: &mut Mmu) { ld_r_hl!(self, mmu, e); }

    fn ld_h_a(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, h, a); }
    fn ld_h_b(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, h, b); }
    fn ld_h_c(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, h, c); }
    fn ld_h_d(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, h, d); }
    fn ld_h_e(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, h, e); }
    fn ld_h_h(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, h, h); }
    fn ld_h_l(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, h, l); }
    fn ld_h_hl(&mut self, mmu: &mut Mmu) { ld_r_hl!(self, mmu, h); }

    fn ld_l_a(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, l, a); }
    fn ld_l_b(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, l, b); }
    fn ld_l_c(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, l, c); }
    fn ld_l_d(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, l, d); }
    fn ld_l_e(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, l, e); }
    fn ld_l_h(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, l, h); }
    fn ld_l_l(&mut self, _mmu: &mut Mmu) { ld_r1_r2!(self, l, l); }
    fn ld_l_hl(&mut self, mmu: &mut Mmu) { ld_r_hl!(self, mmu, l); }

    fn ld_hl_a(&mut self, mmu: &mut Mmu) { ld_hl_r!(self, mmu, a); }
    fn ld_hl_b(&mut self, mmu: &mut Mmu) { ld_hl_r!(self, mmu, b); }
    fn ld_hl_c(&mut self, mmu: &mut Mmu) { ld_hl_r!(self, mmu, c); }
    fn ld_hl_d(&mut self, mmu: &mut Mmu) { ld_hl_r!(self, mmu, d); }
    fn ld_hl_e(&mut self, mmu: &mut Mmu) { ld_hl_r!(self, mmu, e); }
    fn ld_hl_h(&mut self, mmu: &mut Mmu) { ld_hl_r!(self, mmu, h); }
    fn ld_hl_l(&mut self, mmu: &mut Mmu) { ld_hl_r!(self, mmu, l); }
    fn ld_hl_n(&mut self, mmu: &mut Mmu) {
        mmu[self.hl() as usize] = self.next_byte(mmu);
        self.cycles += 12;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::mmu::Mmu;
    use super::opcodes::OPCODES;
    
    fn init(memory: Option<&[u8]>) -> (Cpu, Mmu) {
        let cpu = Cpu::new();
        let mut mmu = Mmu::new();
        if memory.is_some() {
            mmu.set_bytes(memory.unwrap());
        }
        (cpu, mmu)
    }
    fn cycles<F>(cycles: usize, closure: F)
        where F: Fn(&mut Cpu, &mut Mmu)
    {
        let (mut cpu, mut mmu) = init(None);
        test(&mut cpu, &mut mmu, cycles, closure);
    }
    fn test<F>(cpu: &mut Cpu, mmu: &mut Mmu, cycles: usize, closure: F)
        where F: Fn(&mut Cpu, &mut Mmu)
    {
        let prev_cycles = cpu.cycles;
        closure(cpu, mmu);
        assert!(cpu.cycles == prev_cycles + cycles,
            format!("Expected cpu cycles to be {}, got {}", prev_cycles + cycles, cpu.cycles));
    }

    #[test]
    fn test_nop() {
        cycles(8, Cpu::nop);
    }

    fn opcode(opcode: usize) -> opcodes::OpcodeFunction {
        use super::opcodes::OpcodeFunction;
        let func = OPCODES[opcode];
        if func as *const OpcodeFunction as usize == Cpu::nyi as *const OpcodeFunction as usize {
            panic!(format!("Unimplemented opcode: 0x{:X}", opcode));
        }
        func
    }

    #[test]
    fn test_ld_n_nn() {
        macro_rules! test_ld_n_nn(
            ($reg1:ident, $reg2:ident, $func: expr) => {{
                let (mut cpu, mut mmu) = init(Some(&[0,0,1,2]));
                cpu.pc = 2;
                test(&mut cpu, &mut mmu, 12, $func);
                assert!(cpu.$reg1 == mmu[3], format!("Expected {}, got {}", mmu[1], cpu.$reg1));
                assert!(cpu.$reg2 == mmu[2], format!("Expected {}, got {}", mmu[0], cpu.$reg2));
                assert!(cpu.pc == 4, format!("Expected pc=4, got pc={}", cpu.pc));
            }}
        );

        test_ld_n_nn!(b, c, OPCODES[0x1]);
        test_ld_n_nn!(d, e, OPCODES[0x11]);
        test_ld_n_nn!(h, l, OPCODES[0x21]);

        let (mut cpu, mut mmu) = init(Some(&[0,0,1,2]));
        cpu.pc = 2;
        test(&mut cpu, &mut mmu, 12, OPCODES[0x31]);
        assert!(cpu.sp == 513);
        assert!(cpu.pc == 4, format!("Expected pc=4, got pc={}", cpu.pc));
    }

    #[test]
    fn test_ld_r1_r2() {
        macro_rules! test_ld_r1_r2(
            ($r1:ident, $r2:ident, $func:expr) => {{
                let (mut cpu, mut mmu) = init(None);
                cpu.$r2 = 123;
                test(&mut cpu, &mut mmu, 4, $func);
                assert!(cpu.$r1 == cpu.$r2,
                        format!("ld {}, {}: Expected {}, got {}", stringify!($r1), stringify!($r2), cpu.$r2, cpu.$r1));
            }}
        );
        macro_rules! test_ld_r_hl(
            ($r:ident, $func: expr) => {{
                let (mut cpu, mut mmu) = init(None);
                cpu.h = 0x11;
                cpu.l = 0x22;
                mmu[0x1122] = 0x33;
                test(&mut cpu, &mut mmu, 8, $func);
                assert!(cpu.$r == 0x33, format!("ld {}, (hl): Expected {}, got {}", stringify!($r), 0x33, cpu.$r));
            }}
        );
        test_ld_r1_r2!(a, a, opcode(0x7F));
        test_ld_r1_r2!(a, b, opcode(0x78));
        test_ld_r1_r2!(a, c, opcode(0x79));
        test_ld_r1_r2!(a, d, opcode(0x7A));
        test_ld_r1_r2!(a, e, opcode(0x7B));
        test_ld_r1_r2!(a, h, opcode(0x7C));
        test_ld_r1_r2!(a, l, opcode(0x7D));
        test_ld_r_hl!(a, opcode(0x7E));

        test_ld_r1_r2!(b, a, opcode(0x47));
        test_ld_r1_r2!(b, b, opcode(0x40));
        test_ld_r1_r2!(b, c, opcode(0x41));
        test_ld_r1_r2!(b, d, opcode(0x42));
        test_ld_r1_r2!(b, e, opcode(0x43));
        test_ld_r1_r2!(b, h, opcode(0x44));
        test_ld_r1_r2!(b, l, opcode(0x45));
        test_ld_r_hl!(b, opcode(0x46));

        test_ld_r1_r2!(c, a, opcode(0x4F));
        test_ld_r1_r2!(c, b, opcode(0x48));
        test_ld_r1_r2!(c, c, opcode(0x49));
        test_ld_r1_r2!(c, d, opcode(0x4A));
        test_ld_r1_r2!(c, e, opcode(0x4B));
        test_ld_r1_r2!(c, h, opcode(0x4C));
        test_ld_r1_r2!(c, l, opcode(0x4D));
        test_ld_r_hl!(c, opcode(0x4E));

        test_ld_r1_r2!(d, a, opcode(0x57));
        test_ld_r1_r2!(d, b, opcode(0x50));
        test_ld_r1_r2!(d, c, opcode(0x51));
        test_ld_r1_r2!(d, d, opcode(0x52));
        test_ld_r1_r2!(d, e, opcode(0x53));
        test_ld_r1_r2!(d, h, opcode(0x54));
        test_ld_r1_r2!(d, l, opcode(0x55));
        test_ld_r_hl!(d, opcode(0x56));

        test_ld_r1_r2!(e, a, opcode(0x5F));
        test_ld_r1_r2!(e, b, opcode(0x58));
        test_ld_r1_r2!(e, c, opcode(0x59));
        test_ld_r1_r2!(e, d, opcode(0x5A));
        test_ld_r1_r2!(e, e, opcode(0x5B));
        test_ld_r1_r2!(e, h, opcode(0x5C));
        test_ld_r1_r2!(e, l, opcode(0x5D));
        test_ld_r_hl!(e, opcode(0x5E));

        test_ld_r1_r2!(h, a, opcode(0x67));
        test_ld_r1_r2!(h, b, opcode(0x60));
        test_ld_r1_r2!(h, c, opcode(0x61));
        test_ld_r1_r2!(h, d, opcode(0x62));
        test_ld_r1_r2!(h, e, opcode(0x63));
        test_ld_r1_r2!(h, h, opcode(0x64));
        test_ld_r1_r2!(h, l, opcode(0x65));
        test_ld_r_hl!(h, opcode(0x66));

        test_ld_r1_r2!(l, a, opcode(0x6F));
        test_ld_r1_r2!(l, b, opcode(0x68));
        test_ld_r1_r2!(l, c, opcode(0x69));
        test_ld_r1_r2!(l, d, opcode(0x6A));
        test_ld_r1_r2!(l, e, opcode(0x6B));
        test_ld_r1_r2!(l, h, opcode(0x6C));
        test_ld_r1_r2!(l, l, opcode(0x6D));
        test_ld_r_hl!(l, opcode(0x6E));
    }

    #[test]
    fn test_ld_hl_r() {
        macro_rules! test_ld_hl_r(
            ($r:ident, $func: expr) => {{
                let (mut cpu, mut mmu) = init(None);
                cpu.$r = 123;
                cpu.h = 0x11;
                cpu.l = 0x22;
                test(&mut cpu, &mut mmu, 8, $func);
                let value = mmu[cpu.hl() as usize];
                assert!(value == 123,
                        format!("ld (hl), {}: Expected {}, got {}", stringify!($r), 123, value));
            }}
        );
        test_ld_hl_r!(a, opcode(0x77));
        test_ld_hl_r!(b, opcode(0x70));
        test_ld_hl_r!(c, opcode(0x71));
        test_ld_hl_r!(d, opcode(0x72));
        test_ld_hl_r!(e, opcode(0x73));

        // ld_hl_h
        let (mut cpu, mut mmu) = init(None);
        cpu.h = 0x11;
        cpu.l = 0x22;
        test(&mut cpu, &mut mmu, 8, opcode(0x74));
        let value = mmu[cpu.hl() as usize];
        assert!(value == 0x11,
                format!("ld (hl), h: Expected {}, got {}", 0x11, value));

        // ld_hl_l
        let (mut cpu, mut mmu) = init(None);
        cpu.h = 0x11;
        cpu.l = 0x22;
        test(&mut cpu, &mut mmu, 8, opcode(0x75));
        let value = mmu[cpu.hl() as usize];
        assert!(value == 0x22,
                format!("ld (hl), h: Expected {}, got {}", 0x22, value));
    }
}
