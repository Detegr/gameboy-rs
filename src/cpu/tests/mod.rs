use cpu::*;
use mmu::Mmu;

pub fn init(memory: Option<&[u8]>) -> (Cpu, Mmu) {
    let cpu = Cpu::new();
    let mut mmu = Mmu::new();
    if memory.is_some() {
        mmu.set_bytes(memory.unwrap());
    }
    (cpu, mmu)
}
pub fn cycles<F>(cycles: usize, closure: F)
where
    F: Fn(&mut Cpu, &mut Mmu),
{
    let (mut cpu, mut mmu) = init(None);
    test(&mut cpu, &mut mmu, cycles, closure);
}
pub fn test<F>(cpu: &mut Cpu, mmu: &mut Mmu, cycles: usize, closure: F)
where
    F: Fn(&mut Cpu, &mut Mmu),
{
    let prev_cycles = cpu.cycles;
    closure(cpu, mmu);
    assert!(
        cpu.cycles == prev_cycles + cycles,
        format!(
            "Expected cpu cycles to be {}, got {}",
            prev_cycles + cycles,
            cpu.cycles
        )
    );
}
pub fn opcode(opcode: usize) -> opcodes::OpcodeFunction {
    let func = ::cpu::opcodes::OPCODES[opcode];
    func
}

mod adc;
mod add;
mod and;
mod call;
mod cp;
mod daa;
mod dec;
mod inc;
mod jp;
mod jr;
mod ld;
mod misc;
mod nop;
mod notavailable;
mod or;
mod pop;
mod push;
mod ret;
mod rotate;
mod rst;
mod sbc;
mod shift;
mod stop;
mod sub;
mod swap;
mod xor;
