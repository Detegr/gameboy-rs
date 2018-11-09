use cartridge::{Cartridge, MBC1};
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

const RESERVED_ADDRESSES: &'static [(u16, u16, &'static str)] = &[
    (0x0, 0x7, "RST $00"),
    (0x8, 0xF, "RST $08"),
    (0x10, 0x17, "RST $10"),
    (0x18, 0x1F, "RST $18"),
    (0x20, 0x27, "RST $20"),
    (0x28, 0x2F, "RST $28"),
    (0x30, 0x37, "RST $30"),
    (0x38, 0x3F, "RST $38"),
    (0x40, 0x47, "Vertical blank interrupt"),
    (0x48, 0x4F, "LCDC status interrupt"),
    (0x50, 0x57, "Timer overflow interrupt"),
    (0x58, 0x5F, "Serial transfer completion interrupt"),
    (0x60, 0x0FF, "High-to-low of P10-P13 interrupt"),
];

fn is_in_cartridge_area(addr: u16) -> bool {
    addr <= 0x3FFF
}

fn is_in_lower_echo_ram_area(addr: u16) -> bool {
    addr >= 0xC000 && addr <= 0xDE00
}

fn is_in_upper_echo_ram_area(addr: u16) -> bool {
    addr >= 0xE000 && addr <= 0xFE00
}

fn is_reserved(addr: u16) -> Option<&'static str> {
    RESERVED_ADDRESSES
        .into_iter()
        .filter_map(|(start, end, msg)| {
            if addr >= *start && addr <= *end {
                Some(*msg)
            } else {
                None
            }
        })
        .nth(0)
}

pub struct Mmu {
    memory: Box<[u8]>,
    cartridge: Option<Box<Cartridge>>,
}

impl Mmu {
    pub fn new() -> Mmu {
        let mut memory = vec![0; 65536].into_boxed_slice();
        memory[0xFF10] = 0x80;
        memory[0xFF11] = 0xBF;
        memory[0xFF12] = 0xF3;
        memory[0xFF14] = 0xBF;
        memory[0xFF16] = 0x3F;
        memory[0xFF19] = 0xBF;
        memory[0xFF1A] = 0x7F;
        memory[0xFF1B] = 0xFF;
        memory[0xFF1C] = 0x9F;
        memory[0xFF1E] = 0xBF;
        memory[0xFF20] = 0xFF;
        memory[0xFF23] = 0xBF;
        memory[0xFF24] = 0x77;
        memory[0xFF25] = 0xF3;
        memory[0xFF26] = 0xF1;
        memory[0xFF40] = 0x91;
        memory[0xFF47] = 0xFC;
        memory[0xFF48] = 0xFF;
        memory[0xFF49] = 0xFF;
        Mmu {
            memory,
            cartridge: None,
        }
    }
    pub fn load_cartridge<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = fs::File::open(path)?;
        self.load_cartridge_data(file);
        Ok(())
    }
    pub fn load_cartridge_data<R: Read>(&mut self, mut data: R) {
        data.read(&mut self.memory).unwrap();
        const CARTRIDGE_TYPE_LOCATION: u16 = 0x147;
        match self.read_u8(CARTRIDGE_TYPE_LOCATION) {
            0 => {
                info!("Cartridge type 0");
            }
            1 => {
                self.cartridge = Some(Box::new(MBC1::new(&self.memory)));
            }
            ct => {
                panic!("Cartridge type {} not supported yet", ct);
            }
        }
    }
    #[cfg(test)]
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            self.memory[i] = *b;
        }
    }
    pub fn write_u8(&mut self, addr: u16, value: u8) {
        trace!("WRITE[0x{:2X}] = 0x{:02X}", addr, value);
        if is_in_lower_echo_ram_area(addr) {
            self.memory[addr as usize + 0x2000] = value;
        } else if is_in_upper_echo_ram_area(addr) {
            self.memory[addr as usize - 0x2000] = value;
        }
        if let Some(msg) = is_reserved(addr) {
            panic!("Address {:2X} is reserved for {}", addr, msg);
        }
        if let Some(ref mut cartridge) = self.cartridge {
            if is_in_cartridge_area(addr) {
                cartridge.write_u8(addr, value);
                return;
            }
        }
        self.memory[addr as usize] = value;
    }
    pub fn write_u16(&mut self, addr: u16, value: u16) {
        self.write_u8(addr, (value & 0xFF) as u8);
        self.write_u8(addr + 1, ((value >> 8) & 0xFF) as u8);
    }
    pub fn read_u8(&self, addr: u16) -> u8 {
        if let Some(ref cartridge) = self.cartridge {
            if is_in_cartridge_area(addr) {
                let ret = cartridge.read_u8(addr);
                //log!("READ[0x{:2X}], {:2X}", addr, ret);
                return ret;
            }
        }
        let ret = self.memory[addr as usize];
        //log!("READ[0x{:2X}], {:2X}", addr, ret);
        ret
    }
    pub fn read_u16(&self, addr: u16) -> u16 {
        let l = self.read_u8(addr);
        let h = self.read_u8(addr + 1);
        ((h as u16) << 8) | l as u16
    }
}
