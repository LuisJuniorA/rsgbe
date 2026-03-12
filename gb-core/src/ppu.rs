pub struct Ppu {
    // Hardware Registers
    pub lcdc: u8, // 0xFF40
    pub stat: u8, // 0xFF41
    pub scy: u8,  // 0xFF42
    pub scx: u8,  // 0xFF43
    pub ly: u8,   // 0xFF44
    pub lyc: u8,  // 0xFF45
    pub bgp: u8,  // 0xFF47
    pub obp0: u8, // 0xFF48
    pub obp1: u8, // 0xFF49
    pub wy: u8,   // 0xFF4A
    pub wx: u8,   // 0xFF4B

    // Emulator State
    pub cycles: u32,
    pub framebuffer: Box<[u8; 160 * 144 * 4]>, // RGBA pixel buffer
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            lcdc: 0x91, // Standard Game Boy boot values
            stat: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            cycles: 0,
            framebuffer: Box::new([0; 160 * 144 * 4]),
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self, cycles: u8) -> u8 {
        // Returns an interrupt flag (e.g., 0x01 for V-Blank, 0x02 for STAT)
        todo!()
    }
}
