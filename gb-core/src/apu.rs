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
}

impl Apu {
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
        }
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        0
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {}
}
