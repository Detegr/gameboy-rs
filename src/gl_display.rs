extern crate mini_gl_fb;

use self::mini_gl_fb::MiniGlFb;
use gameboy::display::Display;

pub struct GlDisplay {
    buffer: Box<[u8]>,
    output_buf: [u8; 160 * 144],
    fb: MiniGlFb,
}
impl GlDisplay {
    pub fn new() -> GlDisplay {
        let mut fb = mini_gl_fb::gotta_go_fast("GB", 160.0, 144.0);
        fb.change_buffer_format::<u8>(mini_gl_fb::BufferFormat::R);
        fb.use_grayscale_shader();
        GlDisplay {
            buffer: vec![255u8; 256 * 256].into_boxed_slice(),
            fb,
            output_buf: [255u8; 160 * 144],
        }
    }
}
impl Display for GlDisplay {
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
        for y in (0u8..144).rev() {
            for x in 0u8..160 {
                let scrolledx = x.wrapping_add(scrollx);
                let scrolledy = y.wrapping_add(scrolly);
                self.output_buf[i] = self.buffer[scrolledy as usize * 256 + scrolledx as usize];
                i += 1;
            }
        }
        self.fb.update_buffer(&self.output_buf);
    }
}
