use debug_log::log;
use display::Display;

pub struct WasmDisplay {
    dirty: bool,
    buffer: Box<[u8]>,
    output_buf: [u8; 160 * 144],
}
impl WasmDisplay {
    pub fn new() -> WasmDisplay {
        WasmDisplay {
            buffer: vec![255u8; 256 * 256].into_boxed_slice(),
            output_buf: [255u8; 160 * 144],
            dirty: false,
        }
    }
    pub fn buffer(&self) -> [u8; 160 * 144] {
        self.output_buf
    }
    pub fn is_dirty(&mut self) -> bool {
        let ret = self.dirty;
        self.dirty = false;
        ret
    }
}
impl Display for WasmDisplay {
    fn write_scanline(&mut self, y: u8, data: &[u8]) {
        for (i, pixel) in data.into_iter().enumerate() {
            // TODO: Palettes
            let pixel = match pixel {
                0 => 255u8,
                1 => 192u8,
                2 => 96u8,
                3 => 0u8,
                data => unreachable!("Invalid pixel data: {}", data),
            };
            self.buffer[256 * y as usize + i] = pixel;
        }
    }
    fn render_framebuffer(&mut self, scrollx: u8, scrolly: u8) {
        let mut i = 0;
        for y in 0u8..144 {
            for x in 0u8..160 {
                let scrolledx = x.wrapping_add(scrollx);
                let scrolledy = y.wrapping_add(scrolly);
                let val = self.buffer[scrolledy as usize * 256 + scrolledx as usize];
                self.output_buf[i] = val;
                i += 1;
            }
        }
        self.dirty = true;
    }
}
