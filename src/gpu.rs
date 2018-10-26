use mmu::Mmu;

const CPU_MHZ: f64 = 1.05;
const CPU_HZ: f64 = CPU_MHZ * 1000000.0;
const CYCLE_IN_S: f64 = 1.0 / CPU_HZ;

macro_rules! ms_to_cycle_count {
    ($ms: expr) => {
        1.0 / $ms / CYCLE_IN_S
    };
}

/*
if self.cycle_count_exceeded(prev_cycles, VBLANK_CYCLE_INTERVAL) {
    debug!("VBLANK");
}
*/

fn cycle_count_exceeded(prev_cycles: usize, current_cycles: usize, interval: f64) -> bool {
    (prev_cycles as f64 / interval).floor() < (current_cycles as f64 / interval).floor()
}

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
const LY: u16 = 0xFF44;

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
    pub fn step(&mut self, mmu: &mut Mmu, cycles: usize) {
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
                    let mut ly = mmu.read_u8(LY);
                    if ly >= 153 {
                        ly = 0;
                    } else {
                        ly += 1;
                    }
                    mmu.write_u8(LY, ly);
                    if cycles_since_last_vblank > VBLANK_CYCLE_INTERVAL {
                        self.vblank_start_cycles += VBLANK_CYCLE_INTERVAL;
                        self.mode_start_cycles = self.mode_start_cycles
                            - (self.mode_start_cycles % VBLANK_CYCLE_INTERVAL);
                        self.state = GpuState::VBlank;
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
                }
            }
        }
        let mut stat = mmu.read_u8(0xFF41);
        let state_u8: u8 = self.state.into();
        stat &= 0b11111100;
        stat |= state_u8;
        mmu.write_u8(0xFF41, stat);
    }
}
