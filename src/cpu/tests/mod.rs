use cpu::*;
use ram::Ram;

pub fn init(memory: Option<&[u8]>) -> (Cpu, Ram) {
    let cpu = Cpu::new();
    let mut ram = Ram::new();
    if memory.is_some() {
        ram.set_bytes(memory.unwrap());
    }
    (cpu, ram)
}
pub fn cycles<F>(cycles: usize, closure: F)
where
    F: Fn(&mut Cpu, &mut Ram),
{
    let (mut cpu, mut ram) = init(None);
    test(&mut cpu, &mut ram, cycles, closure);
}
pub fn test<F>(cpu: &mut Cpu, ram: &mut Ram, cycles: usize, closure: F)
where
    F: Fn(&mut Cpu, &mut Ram),
{
    let prev_cycles = cpu.cycles;
    closure(cpu, ram);
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
    use cpu::opcodes::{OpcodeFunction, OPCODES};
    let func = OPCODES[opcode];
    if func as *const OpcodeFunction as usize == Cpu::nyi as *const OpcodeFunction as usize {
        panic!(format!("Unimplemented opcode: 0x{:X}", opcode));
    }
    func
}

mod adc;
mod add;
mod and;
mod cp;
mod daa;
mod dec;
mod inc;
mod jr;
mod ld;
mod misc;
mod nop;
mod or;
mod rotate;
mod sbc;
mod stop;
mod sub;
mod xor;
