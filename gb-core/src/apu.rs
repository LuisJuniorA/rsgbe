const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
    [1, 0, 0, 0, 0, 0, 0, 1], // 25%
    [1, 0, 0, 0, 0, 1, 1, 1], // 50%
    [0, 1, 1, 1, 1, 1, 1, 0], // 75%
];

pub struct Channel1 {
    // Registers
    nr10: u8, // 0xFF10 - Sweep register
    nr11: u8, // 0xFF11 - Sound length / Wave pattern duty
    nr12: u8, // 0xFF12 - Volume Envelope
    nr13: u8, // 0xFF13 - Frequency lo (write-only)
    nr14: u8, // 0xFF14 - Frequency hi / Trigger

    // Internal State
    timer: u16,
    duty_pos: u8,
    enabled: bool,
    length_timer: u16,
    volume: u8,
    volume_timer: u8,
    sweep_timer: u8,
    sweep_enabled: bool,
    shadow_freq: u16,
}

impl Channel1 {
    pub fn new() -> Self {
        Self {
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,
            timer: 0,
            duty_pos: 0,
            enabled: false,
            length_timer: 0,
            volume: 0,
            volume_timer: 0,
            sweep_timer: 0,
            sweep_enabled: false,
            shadow_freq: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF10 => self.nr10 | 0x80,
            0xFF11 => self.nr11 | 0x3F,
            0xFF12 => self.nr12,
            0xFF13 => 0xFF,
            0xFF14 => self.nr14 | 0xBF,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF10 => self.nr10 = val,
            0xFF11 => {
                self.nr11 = val;
                self.length_timer = 64 - (val & 0x3F) as u16;
            }
            0xFF12 => self.nr12 = val,
            0xFF13 => self.nr13 = val,
            0xFF14 => {
                self.nr14 = val;
                if val & 0x80 != 0 {
                    self.trigger();
                }
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            let freq = (self.nr13 as u16) | (((self.nr14 & 0x07) as u16) << 8);
            self.timer = (2048 - freq) * 4;
            self.duty_pos = (self.duty_pos + 1) % 8;
        }
    }

    pub fn tick_length(&mut self) {
        if (self.nr14 & 0x40) != 0 && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn tick_sweep(&mut self) {
        if !self.sweep_enabled {
            return;
        }

        self.sweep_timer -= 1;
        if self.sweep_timer == 0 {
            let sweep_period = (self.nr10 >> 4) & 0x07;
            self.sweep_timer = if sweep_period == 0 { 8 } else { sweep_period };

            if sweep_period > 0 {
                let new_freq = self.calculate_sweep_freq();

                if new_freq > 2047 {
                    self.enabled = false;
                } else {
                    let sweep_shift = self.nr10 & 0x07;
                    if sweep_shift > 0 {
                        self.shadow_freq = new_freq;
                        self.nr13 = new_freq as u8;
                        self.nr14 = (self.nr14 & 0xF8) | ((new_freq >> 8) as u8);

                        if self.calculate_sweep_freq() > 2047 {
                            self.enabled = false;
                        }
                    }
                }
            }
        }
    }

    pub fn tick_volume(&mut self) {
        let period = self.nr12 & 0x07;
        if period != 0 {
            if self.volume_timer > 0 {
                self.volume_timer -= 1;
            }
            if self.volume_timer == 0 {
                self.volume_timer = period;
                let increase = (self.nr12 & 0x08) != 0;
                if increase {
                    if self.volume < 15 {
                        self.volume += 1;
                    }
                } else if self.volume > 0 {
                    self.volume -= 1;
                }
            }
        }
    }

    pub fn sample(&self) -> f32 {
        if !self.enabled || (self.nr12 & 0xF8) == 0 {
            return 0.0;
        }
        let duty = (self.nr11 >> 6) as usize;
        let bit = DUTY_TABLE[duty][self.duty_pos as usize];
        let amp = if bit == 0 { -1.0 } else { 1.0 };
        amp * (self.volume as f32 / 15.0)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        let freq = (self.nr13 as u16) | (((self.nr14 & 0x07) as u16) << 8);
        self.timer = (2048 - freq) * 4;
        self.volume = self.nr12 >> 4;
        self.volume_timer = self.nr12 & 0x07;

        self.shadow_freq = freq;
        let sweep_period = (self.nr10 >> 4) & 0x07;
        let sweep_shift = self.nr10 & 0x07;
        self.sweep_timer = if sweep_period == 0 { 8 } else { sweep_period };
        self.sweep_enabled = sweep_period != 0 || sweep_shift != 0;

        if sweep_shift != 0 && self.calculate_sweep_freq() > 2047 {
            self.enabled = false;
        }
    }

    fn calculate_sweep_freq(&self) -> u16 {
        let shift = self.nr10 & 0x07;
        let delta = self.shadow_freq >> shift;
        if (self.nr10 & 0x08) != 0 {
            self.shadow_freq.saturating_sub(delta)
        } else {
            self.shadow_freq + delta
        }
    }
}

pub struct Channel2 {
    // Registers
    nr21: u8, // 0xFF16 - Sound length / Wave pattern duty
    nr22: u8, // 0xFF17 - Volume Envelope
    nr23: u8, // 0xFF18 - Frequency lo (write-only)
    nr24: u8, // 0xFF19 - Frequency hi / Trigger

    // Internal State
    timer: u16,
    duty_pos: u8,
    enabled: bool,
    length_timer: u16,
    volume: u8,
    volume_timer: u8,
}

impl Channel2 {
    pub fn new() -> Self {
        Self {
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,
            timer: 0,
            duty_pos: 0,
            enabled: false,
            length_timer: 0,
            volume: 0,
            volume_timer: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF16 => self.nr21 | 0x3F,
            0xFF17 => self.nr22,
            0xFF18 => 0xFF,
            0xFF19 => self.nr24 | 0xBF,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF16 => {
                self.nr21 = val;
                self.length_timer = 64 - (val & 0x3F) as u16;
            }
            0xFF17 => self.nr22 = val,
            0xFF18 => self.nr23 = val,
            0xFF19 => {
                self.nr24 = val;
                if val & 0x80 != 0 {
                    self.trigger();
                }
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            let freq = (self.nr23 as u16) | (((self.nr24 & 0x07) as u16) << 8);
            self.timer = (2048 - freq) * 4;
            self.duty_pos = (self.duty_pos + 1) % 8;
        }
    }

    pub fn tick_length(&mut self) {
        if (self.nr24 & 0x40) != 0 && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn tick_volume(&mut self) {
        let period = self.nr22 & 0x07;
        if period != 0 {
            if self.volume_timer > 0 {
                self.volume_timer -= 1;
            }
            if self.volume_timer == 0 {
                self.volume_timer = period;
                let increase = (self.nr22 & 0x08) != 0;
                if increase {
                    if self.volume < 15 {
                        self.volume += 1;
                    }
                } else if self.volume > 0 {
                    self.volume -= 1;
                }
            }
        }
    }

    pub fn sample(&self) -> f32 {
        if !self.enabled || (self.nr22 & 0xF8) == 0 {
            return 0.0;
        }
        let duty = (self.nr21 >> 6) as usize;
        let bit = DUTY_TABLE[duty][self.duty_pos as usize];
        let amp = if bit == 0 { -1.0 } else { 1.0 };
        amp * (self.volume as f32 / 15.0)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        let freq = (self.nr23 as u16) | (((self.nr24 & 0x07) as u16) << 8);
        self.timer = (2048 - freq) * 4;
        self.volume = self.nr22 >> 4;
        self.volume_timer = self.nr22 & 0x07;
    }
}

pub struct Channel3 {
    // Registers
    nr30: u8, // 0xFF1A - DAC power (Sound on/off)
    nr31: u8, // 0xFF1B - Sound length
    nr32: u8, // 0xFF1C - Volume code
    nr33: u8, // 0xFF1D - Frequency lo (write-only)
    nr34: u8, // 0xFF1E - Frequency hi / Trigger

    // Wave Pattern RAM
    wave_ram: [u8; 16], // 0xFF30..=0xFF3F - 32 4-bit samples

    // Internal State
    timer: u16,
    pos: u8,
    enabled: bool,
    length_timer: u16,
}

impl Channel3 {
    pub fn new() -> Self {
        Self {
            nr30: 0,
            nr31: 0,
            nr32: 0,
            nr33: 0,
            nr34: 0,
            wave_ram: [0; 16],
            timer: 0,
            pos: 0,
            enabled: false,
            length_timer: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF1A => self.nr30 | 0x7F,
            0xFF1B => 0xFF,
            0xFF1C => self.nr32 | 0x9F,
            0xFF1D => 0xFF,
            0xFF1E => self.nr34 | 0xBF,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF1A => self.nr30 = val,
            0xFF1B => {
                self.nr31 = val;
                self.length_timer = 256 - val as u16;
            }
            0xFF1C => self.nr32 = val,
            0xFF1D => self.nr33 = val,
            0xFF1E => {
                self.nr34 = val;
                if val & 0x80 != 0 {
                    self.trigger();
                }
            }
            _ => {}
        }
    }

    pub fn read_wave_ram(&self, addr: u16) -> u8 {
        self.wave_ram[(addr - 0xFF30) as usize]
    }

    pub fn write_wave_ram(&mut self, addr: u16, val: u8) {
        self.wave_ram[(addr - 0xFF30) as usize] = val;
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            let freq = (self.nr33 as u16) | (((self.nr34 & 0x07) as u16) << 8);
            self.timer = (2048 - freq) * 2;
            self.pos = (self.pos + 1) % 32;
        }
    }

    pub fn tick_length(&mut self) {
        if (self.nr34 & 0x40) != 0 && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn sample(&self) -> f32 {
        if !self.enabled || (self.nr30 & 0x80) == 0 {
            return 0.0;
        }

        let byte = self.wave_ram[(self.pos / 2) as usize];
        let raw = if (self.pos & 1) == 0 { byte >> 4 } else { byte & 0x0F };
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

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn reset(&mut self, wave_ram: [u8; 16]) {
        *self = Self { wave_ram, ..Self::new() };
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_timer == 0 {
            self.length_timer = 256;
        }
        let freq = (self.nr33 as u16) | (((self.nr34 & 0x07) as u16) << 8);
        self.timer = (2048 - freq) * 2;
        self.pos = 0;
    }
}

pub struct Channel4 {
    // Registers
    nr41: u8, // 0xFF20 - Sound length
    nr42: u8, // 0xFF21 - Volume Envelope
    nr43: u8, // 0xFF22 - Polynomial counter (clock shift, LFSR width, divisor)
    nr44: u8, // 0xFF23 - Counter/consecutive / Trigger

    // Internal State
    timer: u16,
    lfsr: u16,
    enabled: bool,
    length_timer: u16,
    volume: u8,
    volume_timer: u8,
}

impl Channel4 {
    pub fn new() -> Self {
        Self {
            nr41: 0,
            nr42: 0,
            nr43: 0,
            nr44: 0,
            timer: 0,
            lfsr: 0x7FFF,
            enabled: false,
            length_timer: 0,
            volume: 0,
            volume_timer: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF20 => 0xFF,
            0xFF21 => self.nr42,
            0xFF22 => self.nr43,
            0xFF23 => self.nr44 | 0xBF,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF20 => {
                self.nr41 = val;
                self.length_timer = 64 - (val & 0x3F) as u16;
            }
            0xFF21 => self.nr42 = val,
            0xFF22 => self.nr43 = val,
            0xFF23 => {
                self.nr44 = val;
                if val & 0x80 != 0 {
                    self.trigger();
                }
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.timer = self.calculate_period();

            let xor_bit = (self.lfsr & 1) ^ ((self.lfsr >> 1) & 1);
            self.lfsr = (self.lfsr >> 1) | (xor_bit << 14);
            if self.nr43 & 0x08 != 0 {
                self.lfsr = (self.lfsr & !(1 << 6)) | (xor_bit << 6);
            }
        }
    }

    pub fn tick_length(&mut self) {
        if (self.nr44 & 0x40) != 0 && self.length_timer > 0 {
            self.length_timer -= 1;
            if self.length_timer == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn tick_volume(&mut self) {
        let period = self.nr42 & 0x07;
        if period != 0 {
            if self.volume_timer > 0 {
                self.volume_timer -= 1;
            }
            if self.volume_timer == 0 {
                self.volume_timer = period;
                let increase = (self.nr42 & 0x08) != 0;
                if increase {
                    if self.volume < 15 {
                        self.volume += 1;
                    }
                } else if self.volume > 0 {
                    self.volume -= 1;
                }
            }
        }
    }

    pub fn sample(&self) -> f32 {
        if !self.enabled || (self.nr42 & 0xF8) == 0 {
            return 0.0;
        }
        let bit = self.lfsr & 1;
        let amp = if bit == 0 { 1.0 } else { -1.0 };
        amp * (self.volume as f32 / 15.0)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_timer == 0 {
            self.length_timer = 64;
        }
        self.timer = self.calculate_period();
        self.lfsr = 0x7FFF;
        self.volume = self.nr42 >> 4;
        self.volume_timer = self.nr42 & 0x07;
    }

    fn calculate_period(&self) -> u16 {
        let divisor_code = self.nr43 & 0x07;
        let divisor = if divisor_code == 0 {
            8
        } else {
            (divisor_code as u16) * 16
        };
        let shift = self.nr43 >> 4;
        divisor << shift
    }
}

pub struct Apu {
    ch1: Channel1,
    ch2: Channel2,
    ch3: Channel3,
    ch4: Channel4,

    // Global Control
    nr50: u8, // 0xFF24 - Master volume & VIN panning
    nr51: u8, // 0xFF25 - Sound panning (routing channels to left/right)
    nr52: u8, // 0xFF26 - Sound master enable (APU power)

    // Frame Sequencer
    fs_timer: u16,
    fs_step: u8,

    // Output Buffer
    pub audio_buffer: Vec<f32>,
    downsample_counter: f32,
}

impl Apu {
    const CPU_FREQ_HZ: f32 = 4_194_304.0;
    const SAMPLE_RATE_HZ: f32 = 44_100.0;

    pub fn new() -> Self {
        Self {
            ch1: Channel1::new(),
            ch2: Channel2::new(),
            ch3: Channel3::new(),
            ch4: Channel4::new(),
            nr50: 0,
            nr51: 0,
            nr52: 0,
            fs_timer: 0,
            fs_step: 0,
            audio_buffer: Vec::new(),
            downsample_counter: 0.0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xFF10..=0xFF14 => self.ch1.read_byte(addr),
            0xFF16..=0xFF19 => self.ch2.read_byte(addr),
            0xFF1A..=0xFF1E => self.ch3.read_byte(addr),
            0xFF20..=0xFF23 => self.ch4.read_byte(addr),

            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => {
                let mut val = self.nr52 & 0x80;
                if self.ch1.is_enabled() {
                    val |= 0x01;
                }
                if self.ch2.is_enabled() {
                    val |= 0x02;
                }
                if self.ch3.is_enabled() {
                    val |= 0x04;
                }
                if self.ch4.is_enabled() {
                    val |= 0x08;
                }
                val | 0x70
            }

            0xFF30..=0xFF3F => self.ch3.read_wave_ram(addr),
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

        if !self.is_on() {
            return;
        }

        match addr {
            0xFF10..=0xFF14 => self.ch1.write_byte(addr, val),
            0xFF16..=0xFF19 => self.ch2.write_byte(addr, val),
            0xFF1A..=0xFF1E => self.ch3.write_byte(addr, val),
            0xFF20..=0xFF23 => self.ch4.write_byte(addr, val),

            0xFF24 => self.nr50 = val,
            0xFF25 => self.nr51 = val,
            0xFF30..=0xFF3F => self.ch3.write_wave_ram(addr, val),
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

            self.ch1.tick();
            self.ch2.tick();
            self.ch3.tick();
            self.ch4.tick();

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

    fn step_frame_sequencer(&mut self) {
        self.fs_step = (self.fs_step + 1) % 8;

        if self.fs_step % 2 == 0 {
            self.ch1.tick_length();
            self.ch2.tick_length();
            self.ch3.tick_length();
            self.ch4.tick_length();
        }

        if self.fs_step == 2 || self.fs_step == 6 {
            self.ch1.tick_sweep();
        }

        if self.fs_step == 7 {
            self.ch1.tick_volume();
            self.ch2.tick_volume();
            self.ch4.tick_volume();
        }
    }

    fn mix_sample(&self) -> (f32, f32) {
        let ch1 = self.ch1.sample();
        let ch2 = self.ch2.sample();
        let ch3 = self.ch3.sample();
        let ch4 = self.ch4.sample();

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

    fn is_on(&self) -> bool {
        self.nr52 & 0x80 != 0
    }

    fn reset_registers(&mut self) {
        let wave_ram = self.ch3.wave_ram;
        let nr52 = self.nr52;
        *self = Self::new();
        self.ch3.wave_ram = wave_ram;
        self.nr52 = nr52;
    }
}
