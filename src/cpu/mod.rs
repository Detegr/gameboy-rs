use byteorder::{ByteOrder, LittleEndian};
use std::default::Default;
use mmu::Mmu;

pub mod opcodes;

#[derive(Default)]
pub struct Cpu {
    _a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    _f: u8,
    pc: usize, // Actually u16 but defined as usize to avoid casting for indexing
    sp: u16,
    cycles: usize,
}
macro_rules! ld_n_nn {
    ($cpu:expr, $mmu:expr, $n1:ident, $n2:ident) => {
        $cpu.$n2=$cpu.next_byte($mmu);
        $cpu.$n1=$cpu.next_byte($mmu);
        $cpu.cycles += 12;
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
        assert!(cpu.cycles == prev_cycles + cycles);
    }

    #[test]
    fn test_nop() {
        cycles(8, Cpu::nop);
    }

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

    #[test]
    fn test_ld_n_nn() {
        test_ld_n_nn!(b, c, OPCODES[1]);
        test_ld_n_nn!(d, e, OPCODES[11]);
        test_ld_n_nn!(h, l, OPCODES[21]);

        let (mut cpu, mut mmu) = init(Some(&[0,0,1,2]));
        cpu.pc = 2;
        test(&mut cpu, &mut mmu, 12, OPCODES[31]);
        assert!(cpu.sp == 513);
        assert!(cpu.pc == 4, format!("Expected pc=4, got pc={}", cpu.pc));
    }
}
