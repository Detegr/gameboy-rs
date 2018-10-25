pub trait Cartridge {
    fn write_u8(&mut self, addr: u16, value: u8);
    fn read_u8(&self, addr: u16) -> u8;
}

// Defaults to 16Mbit rom/8KByte ram mode
pub struct MBC1 {
    memory: Box<[u8]>,
}
impl MBC1 {
    pub fn new(ram: &[u8]) -> MBC1 {
        info!("Cartridge type is MBC1");
        let mut memory = vec![];
        memory.extend_from_slice(&ram[0..0x3FFF]);
        MBC1 {
            memory: memory.into_boxed_slice(),
        }
    }
}

impl Cartridge for MBC1 {
    fn write_u8(&mut self, addr: u16, value: u8) {
        if addr >= 0x2000 && addr <= 0x3FFF {
            unimplemented!("Memory mode selection not implemented yet!");
        }
        if addr >= 0x4000 && addr <= 0x5FFF {
            unimplemented!("ROM address line selection not implemented yet!");
        }
        if addr >= 0x6000 && addr <= 0x7FFF {
            unimplemented!("Memory mode selection not implemented yet!");
        }
        self.memory[addr as usize] = value;
    }
    fn read_u8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
}
