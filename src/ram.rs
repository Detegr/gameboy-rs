use std::io;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};

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
    i: u64,
    memory: Box<[u8]>,
}
impl Ram {
    pub fn new() -> Ram {
        Ram {
            i: 0,
            memory: vec![0; 65536].into_boxed_slice(),
        }
    }

    pub fn read_u8(&mut self, pos: u16) -> io::Result<u8> {
        self.seek(SeekFrom::Start(pos as u64))?;
        let mut buf = [0u8];
        self.read(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u16(&mut self, pos: u16) -> io::Result<u16> {
        self.seek(SeekFrom::Start(pos as u64))?;
        let mut buf = [0u8, 0u8];
        self.read(&mut buf)?;
        Ok((buf[0] as u16) << 8 | buf[1] as u16)
    }

    pub fn write_u8(&mut self, pos: u16, value: u8) -> io::Result<usize> {
        self.seek(SeekFrom::Start(pos as u64))?;
        if let Some(area) = is_reserved(pos) {
            return Err(Error::new(
                ErrorKind::Other,
                format!("cannot write to {:2x}: {}", pos, area),
            ));
        } else {
            if pos >= 0xe000 && pos <= 0xfe00 {
                // 0xe000 - 0xfe00
                // are reflected to
                // 0xc000 - 0xde00
                // and vice versa
                self.memory[pos as usize - 0x2000] = value;
            } else if pos >= 0xc000 && pos <= 0xde00 {
                self.memory[pos as usize + 0x2000] = value;
            }
            self.memory[pos as usize] = value;
            self.i += 1;
            Ok(1)
        }
    }

    pub fn write_u16(&mut self, pos: u16, value: u16) -> io::Result<usize> {
        let low = (value & 0xFF) as u8;
        let high = (value >> 8) as u8;
        self.write_u8(pos, high)?;
        self.write_u8(pos, low)
    }

    #[cfg(test)]
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            self.memory[i] = *b;
        }
    }
}
impl Read for Ram {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for i in 0..buf.len() {
            buf[i] = self.memory[self.i as usize + i]
        }
        self.i += buf.len() as u64;
        Ok(buf.len())
    }
}
impl Write for Ram {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // TODO: This is horribly wasteful
        for byte in buf {
            let i = self.i;
            self.write_u8(i as u16, *byte)?;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Seek for Ram {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(pos) => {
                self.i = pos;
            }
            SeekFrom::End(pos) => {
                if pos < -65535 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Cannot seek before byte 0",
                    ));
                } else if pos > 0 {
                    self.i = 65535;
                } else {
                    self.i = (65535 + pos) as u64;
                }
            }
            SeekFrom::Current(pos) => {
                if pos < self.i as i64 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Cannot seek before byte 0",
                    ));
                }
                self.i = (self.i as i64 + pos) as u64;
                if self.i > 65535 {
                    self.i = 65535;
                }
            }
        }
        Ok(self.i)
    }
}

#[cfg(test)]
impl Deref for Ram {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.memory
    }
}

#[cfg(test)]
impl DerefMut for Ram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.memory
    }
}
