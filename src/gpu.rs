use display::Display;
use mmu::Mmu;

#[derive(Copy, Clone, Debug)]
enum GpuState {
    HBlank,
    VBlank,
    OAM,
    OAMAndDisplayRam,
}
impl Into<u8> for GpuState {
    fn into(self) -> u8 {
        match self {
            GpuState::HBlank => 0b0,
            GpuState::VBlank => 0b01,
            GpuState::OAM => 0b10,
            GpuState::OAMAndDisplayRam => 0b11,
        }
    }
}

const HBLANK_TIME_IN_CYCLES: usize = 51;
const OAM_TIME_IN_CYCLES: usize = 20;
const OAM_AND_DISPLAY_RAM_TIME_IN_CYCLES: usize = 43;
const VBLANK_CYCLE_INTERVAL: usize = 17588 - VBLANK_TIME_IN_CYCLES;
const VBLANK_TIME_IN_CYCLES: usize = 1140;
const LY_REGISTER: u16 = 0xFF44;
const LCDC_REGISTER: u16 = 0xFF40;
const SCROLL_Y_REGISTER: u16 = 0xFF42;
const SCROLL_X_REGISTER: u16 = 0xFF43;

#[derive(Debug)]
pub struct Gpu {
    mode_start_cycles: usize,
    vblank_start_cycles: usize,
    state: GpuState,
}
impl Gpu {
    pub fn new() -> Gpu {
        Gpu {
            mode_start_cycles: 0,
            vblank_start_cycles: 0,
            state: GpuState::HBlank,
        }
    }
    pub fn step<D: Display>(&mut self, display: &mut D, mmu: &mut Mmu, cycles: usize) {
        let cycles_since_mode_start = cycles - self.mode_start_cycles;
        let cycles_since_last_vblank = if cycles > self.vblank_start_cycles {
            cycles - self.vblank_start_cycles
        } else {
            0
        };
        match self.state {
            GpuState::HBlank => {
                if cycles_since_mode_start > HBLANK_TIME_IN_CYCLES {
                    self.mode_start_cycles += HBLANK_TIME_IN_CYCLES;
                    let mut ly = mmu.read_u8(LY_REGISTER);
                    if ly >= 153 {
                        ly = 0;
                    } else {
                        ly += 1;
                    }
                    mmu.write_u8(LY_REGISTER, ly);
                    if cycles_since_last_vblank > VBLANK_CYCLE_INTERVAL {
                        self.vblank_start_cycles += VBLANK_CYCLE_INTERVAL;
                        self.mode_start_cycles = self.mode_start_cycles
                            - (self.mode_start_cycles % VBLANK_CYCLE_INTERVAL);
                        self.state = GpuState::VBlank;

                        display.render_framebuffer(
                            mmu.read_u8(SCROLL_X_REGISTER),
                            mmu.read_u8(SCROLL_Y_REGISTER),
                        );
                    } else {
                        self.state = GpuState::OAM;
                    }
                }
            }
            GpuState::VBlank => {
                if cycles_since_mode_start > VBLANK_TIME_IN_CYCLES {
                    self.mode_start_cycles += VBLANK_TIME_IN_CYCLES;
                    self.state = GpuState::OAM;
                }
            }
            GpuState::OAM => {
                if cycles_since_mode_start > OAM_TIME_IN_CYCLES {
                    self.mode_start_cycles += OAM_TIME_IN_CYCLES;
                    self.state = GpuState::OAMAndDisplayRam;
                }
            }
            GpuState::OAMAndDisplayRam => {
                if cycles_since_mode_start > OAM_AND_DISPLAY_RAM_TIME_IN_CYCLES {
                    self.mode_start_cycles += OAM_AND_DISPLAY_RAM_TIME_IN_CYCLES;
                    self.state = GpuState::HBlank;

                    self.write_scanline(
                        display,
                        mmu,
                        LCDC(mmu.read_u8(LCDC_REGISTER)),
                        mmu.read_u8(LY_REGISTER),
                    );
                }
            }
        }
        let mut stat = mmu.read_u8(0xFF41);
        let state_u8: u8 = self.state.into();
        stat &= 0b11111100;
        stat |= state_u8;
        mmu.write_u8(0xFF41, stat);
    }

    fn write_scanline<D: Display>(&self, display: &mut D, mmu: &Mmu, lcdc: LCDC, ly: u8) {
        match lcdc.lcd_control_operation() {
            LCDCField::Operation => {}
            LCDCField::StopCompletely => {}
            _ => unreachable!(),
        }

        let window_tilemap_display_start = match lcdc.window_tilemap_display() {
            LCDCField::_9800_9BFF => 0x9800,
            LCDCField::_9C00_9FFF => 0x9C00,
            _ => unreachable!(),
        };

        let tile_data_start = match lcdc.bg_and_window_tile_data_select() {
            LCDCField::_8000_8FFF => 0x8000,
            LCDCField::_8800_97FF => 0x8800,
            _ => unreachable!(),
        };

        let bg_tilemap_display_start = match lcdc.bg_tile_map_display_select() {
            LCDCField::_9800_9BFF => 0x9800,
            LCDCField::_9C00_9FFF => {
                unimplemented!("Bg tile map display 0x9C00 - 0x9CFF");
            }
            _ => unreachable!(),
        };

        if lcdc.window_display() {
            unimplemented!("Window display");
        }

        let tilemap_row = (ly / 8) as u16;
        let mut line_buf = [0u8; 256];
        for x in 0..32 {
            let mut tile = mmu.read_u8(bg_tilemap_display_start + (tilemap_row * 32) + x);
            if tile_data_start == 0x8800 {
                // Indexes are from -128 to 127
                tile = tile.wrapping_add(128);
            }
            let ydiff = (ly % 8) as u16;
            let tile_pixel_data_start = tile_data_start + (tile as u16 * 16) + (2 * ydiff);

            let first = mmu.read_u8(tile_pixel_data_start);
            let second = mmu.read_u8(tile_pixel_data_start + 1);
            line_buf[x as usize * 8 + 0] = (first & 0x80) >> 6 | (second & 0x80) >> 7;
            line_buf[x as usize * 8 + 1] = (first & 0x40) >> 5 | (second & 0x40) >> 6;
            line_buf[x as usize * 8 + 2] = (first & 0x20) >> 4 | (second & 0x20) >> 5;
            line_buf[x as usize * 8 + 3] = (first & 0x10) >> 3 | (second & 0x10) >> 4;
            line_buf[x as usize * 8 + 4] = (first & 0x8) >> 2 | (second & 0x8) >> 3;
            line_buf[x as usize * 8 + 5] = (first & 0x4) >> 1 | (second & 0x4) >> 2;
            line_buf[x as usize * 8 + 6] = (first & 0x2) | (second & 0x2) >> 1;
            line_buf[x as usize * 8 + 7] = (first & 0x1) << 1 | second & 0x1;
        }
        display.write_scanline(ly, &line_buf);
    }
}

pub enum LCDCField {
    Operation,
    StopCompletely,
    _8000_8FFF,
    _8800_97FF,
    _8x16,
    _8x8,
    _9800_9BFF,
    _9C00_9FFF,
}
pub struct LCDC(pub u8);
impl LCDC {
    fn lcd_control_operation(&self) -> LCDCField {
        if self.0 & 0x80 == 0 {
            LCDCField::StopCompletely
        } else {
            LCDCField::Operation
        }
    }
    fn window_tilemap_display(&self) -> LCDCField {
        if self.0 & 0x40 == 0 {
            LCDCField::_9800_9BFF
        } else {
            LCDCField::_9C00_9FFF
        }
    }
    fn window_display(&self) -> bool {
        self.0 & 0x20 != 0
    }
    fn bg_and_window_tile_data_select(&self) -> LCDCField {
        if self.0 & 0x10 == 0 {
            LCDCField::_8800_97FF
        } else {
            LCDCField::_8000_8FFF
        }
    }
    fn bg_tile_map_display_select(&self) -> LCDCField {
        if self.0 & 0x08 == 0 {
            LCDCField::_9800_9BFF
        } else {
            LCDCField::_9C00_9FFF
        }
    }
    fn sprite_size(&self) -> LCDCField {
        if self.0 & 0x04 == 0 {
            LCDCField::_8x8
        } else {
            LCDCField::_8x16
        }
    }
    fn sprite_display(&self) -> bool {
        self.0 & 0x02 != 0
    }
    fn bg_and_window_display(&self) -> bool {
        self.0 & 0x01 != 0
    }
}
