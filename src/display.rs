pub trait Display {
    fn write_scanline(&mut self, y: u8, data: &[u8]);
    fn render_framebuffer(&mut self, scrollx: u8, scrolly: u8);
}

pub struct DebugDisplay;
impl Display for DebugDisplay {
    fn write_scanline(&mut self, y: u8, _data: &[u8]) {
        debug!("write_scanline, y: {}", y);
    }
    fn render_framebuffer(&mut self, scrollx: u8, scrolly: u8) {
        debug!("render_framebuffer, x: {}, y: {}", scrollx, scrolly);
    }
}
