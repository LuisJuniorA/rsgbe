pub struct Apu {
    // Channel 1: Pulse with sweep
    nr10: u8, // 0xFF10 - Sweep register
    nr11: u8, // 0xFF11 - Sound length / Wave pattern duty
    nr12: u8, // 0xFF12 - Volume Envelope
    nr13: u8, // 0xFF13 - Frequency lo (write-only)
    nr14: u8, // 0xFF14 - Frequency hi / Trigger

    // Channel 2: Pulse
    nr21: u8, // 0xFF16 - Sound length / Wave pattern duty
    nr22: u8, // 0xFF17 - Volume Envelope
    nr23: u8, // 0xFF18 - Frequency lo (write-only)
    nr24: u8, // 0xFF19 - Frequency hi / Trigger

    // Channel 3: Wave output
    nr30: u8, // 0xFF1A - DAC power (Sound on/off)
    nr31: u8, // 0xFF1B - Sound length
    nr32: u8, // 0xFF1C - Volume code
    nr33: u8, // 0xFF1D - Frequency lo (write-only)
    nr34: u8, // 0xFF1E - Frequency hi / Trigger

    // Channel 4: Noise
    nr41: u8, // 0xFF20 - Sound length
    nr42: u8, // 0xFF21 - Volume Envelope
    nr43: u8, // 0xFF22 - Polynomial counter (clock shift, LFSR width, divisor)
    nr44: u8, // 0xFF23 - Counter/consecutive / Trigger

    // Global Control
    nr50: u8, // 0xFF24 - Master volume & VIN panning
    nr51: u8, // 0xFF25 - Sound panning (routing channels to left/right)
    nr52: u8, // 0xFF26 - Sound master enable (APU power)

    // Wave Pattern RAM
    wave_ram: [u8; 16], // 0xFF30..=0xFF3F - 32 4-bit samples for Channel 3

    // Frame Sequencer
    fs_timer: u16,
    fs_step: u8,

    // Channel 1 Internal
    ch1_timer: u16,
    ch1_duty_pos: u8,
    ch1_enabled: bool,
    ch1_length_timer: u16,
    ch1_volume: u8,
    ch1_volume_timer: u8,
    ch1_sweep_timer: u8,
    ch1_sweep_enabled: bool,
    ch1_shadow_freq: u16,

    // Channel 2 Internal
    ch2_timer: u16,
    ch2_duty_pos: u8,
    ch2_enabled: bool,
    ch2_length_timer: u16,
    ch2_volume: u8,
    ch2_volume_timer: u8,

    // Channel 3 Internal
    ch3_timer: u16,
    ch3_pos: u8,
    ch3_enabled: bool,
    ch3_length_timer: u16,

    // Channel 4 Internal
    ch4_timer: u16,
    ch4_lfsr: u16,
    ch4_enabled: bool,
    ch4_length_timer: u16,
    ch4_volume: u8,
    ch4_volume_timer: u8,

    // Output Buffer
    pub audio_buffer: Vec<f32>,
    downsample_counter: f32,
}

impl Apu {
    const CPU_FREQ_HZ: f32 = 4_194_304.0;
    const SAMPLE_RATE_HZ: f32 = 44_100.0;
    const DUTY_TABLE: [[u8; 8]; 4] = [
        [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
        [1, 0, 0, 0, 0, 0, 0, 1], // 25%
        [1, 0, 0, 0, 0, 1, 1, 1], // 50%
        [0, 1, 1, 1, 1, 1, 1, 0], // 75%
    ];

    pub fn new() -> Self {
        Self {
            // Channel 1
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,

            // Channel 2
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,

            // Channel 3
            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,

            // Channel 4
            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,

            // Global Control
            nr50: 0,
            nr51: 0,
            nr52: 0,

            // Wave RAM
            wave_ram: [0; 16],

            fs_timer: 0,
            fs_step: 0,

            ch1_timer: 0,
            ch1_duty_pos: 0,
            ch1_enabled: false,
            ch1_length_timer: 0,
            ch1_volume: 0,
            ch1_volume_timer: 0,
            ch1_sweep_timer: 0,
            ch1_sweep_enabled: false,
            ch1_shadow_freq: 0,

            ch2_timer: 0,
            ch2_duty_pos: 0,
            ch2_enabled: false,
            ch2_length_timer: 0,
            ch2_volume: 0,
            ch2_volume_timer: 0,

            ch3_timer: 0,
            ch3_pos: 0,
            ch3_enabled: false,
            ch3_length_timer: 0,

            ch4_timer: 0,
            ch4_lfsr: 0x7FFF,
            ch4_enabled: false,
            ch4_length_timer: 0,
            ch4_volume: 0,
            ch4_volume_timer: 0,

            audio_buffer: Vec::new(),
            downsample_counter: 0.0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF10 => self.nr10 | 0x80,
            0xFF11 => self.nr11 | 0x3F,
            0xFF12 => self.nr12,
            0xFF13 => 0xFF,
            0xFF14 => self.nr14 | 0xBF,

            0xFF16 => self.nr21 | 0x3F,
            0xFF17 => self.nr22,
            0xFF18 => 0xFF,
            0xFF19 => self.nr24 | 0xBF,

            0xFF1A => self.nr30 | 0x7F,
            0xFF1B => 0xFF,
            0xFF1C => self.nr32 | 0x9F,
            0xFF1D => 0xFF,
            0xFF1E => self.nr34 | 0xBF,

            0xFF20 => 0xFF,
            0xFF21 => self.nr42,
            0xFF22 => self.nr43,
            0xFF23 => self.nr44 | 0xBF,

            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => {
                let mut val = self.nr52 & 0x80;
                if self.ch1_enabled {
                    val |= 0x01;
                }
                if self.ch2_enabled {
                    val |= 0x02;
                }
                if self.ch3_enabled {
                    val |= 0x04;
                }
                if self.ch4_enabled {
                    val |= 0x08;
                }
                val | 0x70
            }

            0xFF30..=0xFF3F => self.wave_ram[(addr - 0xFF30) as usize],
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        if addr == 0xFF26 {
            let was_on = self.is_on();
            self.nr52 = (self.nr52 & 0x0F) | (val & 0x80);
            if was_on && !self.is_on() {
                self.reset_registers();
            }
            return;
        }

        if (0xFF30..=0xFF3F).contains(&addr) {
            self.wave_ram[(addr - 0xFF30) as usize] = val;
            return;
        }

        if !self.is_on() {
            return;
        }

        match addr {
            0xFF10 => self.nr10 = val,
            0xFF11 => {
                self.nr11 = val;
                self.ch1_length_timer = 64 - (val & 0x3F) as u16;
            }
            0xFF12 => self.nr12 = val,
            0xFF13 => self.nr13 = val,
            0xFF14 => {
                self.nr14 = val;
                if val & 0x80 != 0 {
                    self.trigger_ch1();
                }
            }

            0xFF16 => {
                self.nr21 = val;
                self.ch2_length_timer = 64 - (val & 0x3F) as u16;
            }
            0xFF17 => self.nr22 = val,
            0xFF18 => self.nr23 = val,
            0xFF19 => {
                self.nr24 = val;
                if val & 0x80 != 0 {
                    self.trigger_ch2();
                }
            }

            0xFF1A => self.nr30 = val,
            0xFF1B => {
                self.nr31 = val;
                self.ch3_length_timer = 256 - val as u16;
            }
            0xFF1C => self.nr32 = val,
            0xFF1D => self.nr33 = val,
            0xFF1E => {
                self.nr34 = val;
                if val & 0x80 != 0 {
                    self.trigger_ch3();
                }
            }

            0xFF20 => {
                self.nr41 = val;
                self.ch4_length_timer = 64 - (val & 0x3F) as u16;
            }
            0xFF21 => self.nr42 = val,
            0xFF22 => self.nr43 = val,
            0xFF23 => {
                self.nr44 = val;
                if val & 0x80 != 0 {
                    self.trigger_ch4();
                }
            }

            0xFF24 => self.nr50 = val,
            0xFF25 => self.nr51 = val,
            _ => {}
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        if !self.is_on() {
            return;
        }

        for _ in 0..cycles {
            self.fs_timer += 1;
            if self.fs_timer >= 8192 {
                self.fs_timer = 0;
                self.step_frame_sequencer();
            }

            self.tick_ch1();
            self.tick_ch2();
            self.tick_ch3();
            self.tick_ch4();

            self.downsample_counter += 1.0;
            let cycles_per_sample = Self::CPU_FREQ_HZ / Self::SAMPLE_RATE_HZ;
            while self.downsample_counter >= cycles_per_sample {
                self.downsample_counter -= cycles_per_sample;
                let (left, right) = self.mix_sample();
                self.audio_buffer.push(left);
                self.audio_buffer.push(right);
            }
        }
    }

    fn tick_ch1(&mut self) {
        if self.ch1_timer > 0 {
            self.ch1_timer -= 1;
        }
        if self.ch1_timer == 0 {
            let freq = (self.nr13 as u16) | (((self.nr14 & 0x07) as u16) << 8);
            self.ch1_timer = (2048 - freq) * 4;
            self.ch1_duty_pos = (self.ch1_duty_pos + 1) % 8;
        }
    }

    fn tick_ch2(&mut self) {
        if self.ch2_timer > 0 {
            self.ch2_timer -= 1;
        }
        if self.ch2_timer == 0 {
            let freq = (self.nr23 as u16) | (((self.nr24 & 0x07) as u16) << 8);
            self.ch2_timer = (2048 - freq) * 4;
            self.ch2_duty_pos = (self.ch2_duty_pos + 1) % 8;
        }
    }

    fn tick_ch3(&mut self) {
        if self.ch3_timer > 0 {
            self.ch3_timer -= 1;
        }
        if self.ch3_timer == 0 {
            let freq = (self.nr33 as u16) | (((self.nr34 & 0x07) as u16) << 8);
            self.ch3_timer = (2048 - freq) * 2;
            self.ch3_pos = (self.ch3_pos + 1) % 32;
        }
    }

    fn tick_ch4(&mut self) {
        if self.ch4_timer > 0 {
            self.ch4_timer -= 1;
        }
        if self.ch4_timer == 0 {
            self.ch4_timer = self.calculate_ch4_period();

            let xor_bit = (self.ch4_lfsr & 1) ^ ((self.ch4_lfsr >> 1) & 1);
            self.ch4_lfsr = (self.ch4_lfsr >> 1) | (xor_bit << 14);
            if self.nr43 & 0x08 != 0 {
                self.ch4_lfsr = (self.ch4_lfsr & !(1 << 6)) | (xor_bit << 6);
            }
        }
    }

    fn step_frame_sequencer(&mut self) {
        self.fs_step = (self.fs_step + 1) % 8;

        if self.fs_step % 2 == 0 {
            self.tick_length();
        }

        if self.fs_step == 2 || self.fs_step == 6 {
            self.tick_sweep();
        }

        if self.fs_step == 7 {
            self.tick_volume();
        }
    }

    fn tick_length(&mut self) {
        if (self.nr14 & 0x40) != 0 && self.ch1_length_timer > 0 {
            self.ch1_length_timer -= 1;
            if self.ch1_length_timer == 0 {
                self.ch1_enabled = false;
            }
        }
        if (self.nr24 & 0x40) != 0 && self.ch2_length_timer > 0 {
            self.ch2_length_timer -= 1;
            if self.ch2_length_timer == 0 {
                self.ch2_enabled = false;
            }
        }
        if (self.nr34 & 0x40) != 0 && self.ch3_length_timer > 0 {
            self.ch3_length_timer -= 1;
            if self.ch3_length_timer == 0 {
                self.ch3_enabled = false;
            }
        }
        if (self.nr44 & 0x40) != 0 && self.ch4_length_timer > 0 {
            self.ch4_length_timer -= 1;
            if self.ch4_length_timer == 0 {
                self.ch4_enabled = false;
            }
        }
    }

    fn tick_sweep(&mut self) {
        if !self.ch1_sweep_enabled {
            return;
        }

        self.ch1_sweep_timer -= 1;
        if self.ch1_sweep_timer == 0 {
            let sweep_period = (self.nr10 >> 4) & 0x07;
            self.ch1_sweep_timer = if sweep_period == 0 { 8 } else { sweep_period };

            if sweep_period > 0 {
                let new_freq = self.calculate_sweep_freq();

                if new_freq > 2047 {
                    self.ch1_enabled = false;
                } else {
                    let sweep_shift = self.nr10 & 0x07;
                    if sweep_shift > 0 {
                        self.ch1_shadow_freq = new_freq;
                        self.nr13 = new_freq as u8;
                        self.nr14 = (self.nr14 & 0xF8) | ((new_freq >> 8) as u8);

                        // Check if the new frequency is out of range
                        if self.calculate_sweep_freq() > 2047 {
                            self.ch1_enabled = false;
                        }
                    }
                }
            }
        }
    }

    fn tick_volume(&mut self) {
        // Channel 1
        let ch1_period = self.nr12 & 0x07;
        if ch1_period != 0 {
            if self.ch1_volume_timer > 0 {
                self.ch1_volume_timer -= 1;
            }
            if self.ch1_volume_timer == 0 {
                self.ch1_volume_timer = ch1_period;
                let increase = (self.nr12 & 0x08) != 0;
                if increase {
                    if self.ch1_volume < 15 {
                        self.ch1_volume += 1;
                    }
                } else if self.ch1_volume > 0 {
                    self.ch1_volume -= 1;
                }
            }
        }

        // Channel 2
        let ch2_period = self.nr22 & 0x07;
        if ch2_period != 0 {
            if self.ch2_volume_timer > 0 {
                self.ch2_volume_timer -= 1;
            }
            if self.ch2_volume_timer == 0 {
                self.ch2_volume_timer = ch2_period;
                let increase = (self.nr22 & 0x08) != 0;
                if increase {
                    if self.ch2_volume < 15 {
                        self.ch2_volume += 1;
                    }
                } else if self.ch2_volume > 0 {
                    self.ch2_volume -= 1;
                }
            }
        }

        // Channel 4
        let ch4_period = self.nr42 & 0x07;
        if ch4_period != 0 {
            if self.ch4_volume_timer > 0 {
                self.ch4_volume_timer -= 1;
            }
            if self.ch4_volume_timer == 0 {
                self.ch4_volume_timer = ch4_period;
                let increase = (self.nr42 & 0x08) != 0;
                if increase {
                    if self.ch4_volume < 15 {
                        self.ch4_volume += 1;
                    }
                } else if self.ch4_volume > 0 {
                    self.ch4_volume -= 1;
                }
            }
        }
    }

    fn mix_sample(&self) -> (f32, f32) {
        let ch1 = self.sample_ch1();
        let ch2 = self.sample_ch2();
        let ch3 = self.sample_ch3();
        let ch4 = self.sample_ch4();

        let mut left = 0.0;
        let mut right = 0.0;

        if (self.nr51 & 0x10) != 0 {
            left += ch1;
        }
        if (self.nr51 & 0x20) != 0 {
            left += ch2;
        }
        if (self.nr51 & 0x40) != 0 {
            left += ch3;
        }
        if (self.nr51 & 0x80) != 0 {
            left += ch4;
        }
        if (self.nr51 & 0x01) != 0 {
            right += ch1;
        }
        if (self.nr51 & 0x02) != 0 {
            right += ch2;
        }
        if (self.nr51 & 0x04) != 0 {
            right += ch3;
        }
        if (self.nr51 & 0x08) != 0 {
            right += ch4;
        }

        let left_vol = ((self.nr50 >> 4) & 0x07) as f32 / 7.0;
        let right_vol = (self.nr50 & 0x07) as f32 / 7.0;

        let mix_scale = 0.25;
        left = left * left_vol * mix_scale;
        right = right * right_vol * mix_scale;

        (left, right)
    }

    fn sample_ch1(&self) -> f32 {
        if !self.ch1_enabled || (self.nr12 & 0xF8) == 0 {
            return 0.0;
        }
        let duty = (self.nr11 >> 6) as usize;
        let bit = Self::DUTY_TABLE[duty][self.ch1_duty_pos as usize];
        let amp = if bit == 0 { -1.0 } else { 1.0 };
        amp * (self.ch1_volume as f32 / 15.0)
    }

    fn sample_ch2(&self) -> f32 {
        if !self.ch2_enabled || (self.nr22 & 0xF8) == 0 {
            return 0.0;
        }
        let duty = (self.nr21 >> 6) as usize;
        let bit = Self::DUTY_TABLE[duty][self.ch2_duty_pos as usize];
        let amp = if bit == 0 { -1.0 } else { 1.0 };
        amp * (self.ch2_volume as f32 / 15.0)
    }

    fn sample_ch3(&self) -> f32 {
        if !self.ch3_enabled || (self.nr30 & 0x80) == 0 {
            return 0.0;
        }

        let byte = self.wave_ram[(self.ch3_pos / 2) as usize];
        let raw = if (self.ch3_pos & 1) == 0 {
            byte >> 4
        } else {
            byte & 0x0F
        };
        let mut sample = (raw as f32 / 15.0) * 2.0 - 1.0;

        let volume_code = (self.nr32 >> 5) & 0x03;
        sample *= match volume_code {
            0 => 0.0,
            1 => 1.0,
            2 => 0.5,
            3 => 0.25,
            _ => 0.0,
        };

        sample
    }

    fn sample_ch4(&self) -> f32 {
        if !self.ch4_enabled || (self.nr42 & 0xF8) == 0 {
            return 0.0;
        }
        let bit = self.ch4_lfsr & 1;
        let amp = if bit == 0 { 1.0 } else { -1.0 };
        amp * (self.ch4_volume as f32 / 15.0)
    }

    fn is_on(&self) -> bool {
        self.nr52 & 0x80 != 0
    }

    fn reset_registers(&mut self) {
        *self = Self {
            wave_ram: self.wave_ram,
            nr52: self.nr52,
            ..Self::new()
        };
    }

    fn trigger_ch1(&mut self) {
        self.ch1_enabled = true;
        if self.ch1_length_timer == 0 {
            self.ch1_length_timer = 64;
        }
        let freq = (self.nr13 as u16) | (((self.nr14 & 0x07) as u16) << 8);
        self.ch1_timer = (2048 - freq) * 4;
        self.ch1_volume = self.nr12 >> 4;
        self.ch1_volume_timer = self.nr12 & 0x07;

        self.ch1_shadow_freq = freq;
        let sweep_period = (self.nr10 >> 4) & 0x07;
        let sweep_shift = self.nr10 & 0x07;
        self.ch1_sweep_timer = if sweep_period == 0 { 8 } else { sweep_period };
        self.ch1_sweep_enabled = sweep_period != 0 || sweep_shift != 0;

        if sweep_shift != 0 && self.calculate_sweep_freq() > 2047 {
            self.ch1_enabled = false;
        }
    }

    fn trigger_ch2(&mut self) {
        self.ch2_enabled = true;
        if self.ch2_length_timer == 0 {
            self.ch2_length_timer = 64;
        }
        let freq = (self.nr23 as u16) | (((self.nr24 & 0x07) as u16) << 8);
        self.ch2_timer = (2048 - freq) * 4;
        self.ch2_volume = self.nr22 >> 4;
        self.ch2_volume_timer = self.nr22 & 0x07;
    }

    fn trigger_ch3(&mut self) {
        self.ch3_enabled = true;
        if self.ch3_length_timer == 0 {
            self.ch3_length_timer = 256;
        }
        let freq = (self.nr33 as u16) | (((self.nr34 & 0x07) as u16) << 8);
        self.ch3_timer = (2048 - freq) * 2;
        self.ch3_pos = 0;
    }

    fn trigger_ch4(&mut self) {
        self.ch4_enabled = true;
        if self.ch4_length_timer == 0 {
            self.ch4_length_timer = 64;
        }
        self.ch4_timer = self.calculate_ch4_period();
        self.ch4_lfsr = 0x7FFF;
        self.ch4_volume = self.nr42 >> 4;
        self.ch4_volume_timer = self.nr42 & 0x07;
    }

    fn calculate_ch4_period(&self) -> u16 {
        let divisor_code = self.nr43 & 0x07;
        let divisor = if divisor_code == 0 {
            8
        } else {
            (divisor_code as u16) * 16
        };
        let shift = self.nr43 >> 4;
        divisor << shift
    }

    fn calculate_sweep_freq(&self) -> u16 {
        let shift = self.nr10 & 0x07;
        let delta = self.ch1_shadow_freq >> shift;
        if (self.nr10 & 0x08) != 0 {
            self.ch1_shadow_freq.saturating_sub(delta)
        } else {
            self.ch1_shadow_freq + delta
        }
    }
}
