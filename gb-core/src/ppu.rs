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

    pub fn step(&mut self, cycles: u8, vram: &[u8], oam: &[u8]) -> u8 {
        // Returns an interrupt flag (e.g., 0x01 for V-Blank, 0x02 for STAT)
        todo!()
    }

    fn draw_scanline(&mut self, vram: &[u8], _oam: &[u8]) {
        if self.lcdc & 0x01 == 0 {
            return;
        }

        let y_pos = self.scy.wrapping_add(self.ly);
        let tile_row = (y_pos / 8) as u16;
        let bg_map_base = if (self.lcdc & 0x08) != 0 {
            0x1C00
        } else {
            0x1800
        };

        let unsigned_addressing = (self.lcdc & 0x10) != 0;

        for x in 0..160 {
            let x_pos = self.scx.wrapping_add(x as u8);
            let tile_col = (x_pos / 8) as u16;

            let tile_address = bg_map_base + (tile_row * 32) + tile_col;
            let tile_id = vram[tile_address as usize];

            let tile_data_address = if unsigned_addressing {
                tile_id as u16 * 16
            } else {
                let signed_id = tile_id as i8;
                let offset = (signed_id as i16) * 16;
                (0x1000_i16 + offset) as u16
            };

            let line_within_tile = (y_pos & 7) as u16;

            let pixel_data_address = tile_data_address + (line_within_tile * 2);

            let byte1 = vram[pixel_data_address as usize];
            let byte2 = vram[(pixel_data_address + 1) as usize];

            // TODO: Extract the exact pixel color from byte1 and byte2 using x_pos
        }
    }
}
