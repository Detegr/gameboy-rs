use std::ops::{Index, IndexMut, Range};

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

pub struct Ram {
    memory: Box<[u8]>,
}
impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: vec![0; 65536].into_boxed_slice(),
        }
    }
    #[cfg(test)]
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            self.memory[i] = *b;
        }
    }
}
impl Index<Range<u16>> for Ram {
    type Output = [u8];
    fn index(&self, index: Range<u16>) -> &Self::Output {
        if (index.start >= 0xE000 && index.start <= 0xFE00)
            || (index.start >= 0xC000 && index.start <= 0xDE00)
        {
            panic!("Cannot index echo ram area");
        }
        if (index.end >= 0xE000 && index.end <= 0xFE00)
            || (index.end >= 0xC000 && index.end <= 0xDE00)
        {
            panic!("Cannot index echo ram area");
        }
        &self.memory[index.start as usize..index.end as usize]
    }
}
impl IndexMut<Range<u16>> for Ram {
    fn index_mut(&mut self, index: Range<u16>) -> &mut Self::Output {
        if (index.start >= 0xE000 && index.start <= 0xFE00)
            || (index.start >= 0xC000 && index.start <= 0xDE00)
        {
            panic!("Cannot mutably index echo ram area");
        }
        if (index.end >= 0xE000 && index.end <= 0xFE00)
            || (index.end >= 0xC000 && index.end <= 0xDE00)
        {
            panic!("Cannot mutably index echo ram area");
        }
        &mut self.memory[index.start as usize..index.end as usize]
    }
}
impl Index<u16> for Ram {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        // 0xC000 - 0xDE00 is mapped to
        // 0xE000 - 0xFE00 also
        let i = if index >= 0xE000 && index <= 0xFE00 {
            index - 0x2000
        } else {
            index
        };
        &self.memory[i as usize]
    }
}
impl IndexMut<u16> for Ram {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let i = if index >= 0xE000 && index <= 0xFE00 {
            index - 0x2000
        } else {
            index
        };
        if let Some(msg) = is_reserved(i) {
            panic!("Address {:2X} is reserved for {}", i, msg);
        }
        &mut self.memory[i as usize]
    }
}
