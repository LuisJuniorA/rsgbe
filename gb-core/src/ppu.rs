#[derive(Clone, Copy)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

pub struct Ppu {
    // Hardware Registers
    pub lcdc: u8, // 0xFF40
    pub stat: u8, // 0xFF41
    pub scy: u8,  // 0xFF42
    pub scx: u8,  // 0xFF43
    pub ly: u8,   // 0xFF44
    pub lyc: u8,  // 0xFF45
    pub dma: u8,  // 0xFF46
    pub bgp: u8,  // 0xFF47
    pub obp0: u8, // 0xFF48
    pub obp1: u8, // 0xFF49
    pub wy: u8,   // 0xFF4A
    pub wx: u8,   // 0xFF4B

    // Emulator State
    pub cycles: u32,
    pub framebuffer: Box<[u8; 160 * 144 * 4]>, // RGBA pixel buffer
    pub stat_irq_line: bool,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            lcdc: 0x91,
            stat: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            cycles: 0,
            framebuffer: Box::new([0; 160 * 144 * 4]),
            stat_irq_line: false,
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_mode(&self) -> PpuMode {
        match self.stat & 0x03 {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamScan,
            3 => PpuMode::Drawing,
            _ => unreachable!(),
        }
    }

    fn set_mode(&mut self, mode: PpuMode) -> bool {
        self.stat = (self.stat & 0xFC) | (mode as u8);

        let interrupt_enable = match mode {
            PpuMode::HBlank => (self.stat & 0x08) != 0,
            PpuMode::VBlank => (self.stat & 0x10) != 0,
            PpuMode::OamScan => (self.stat & 0x20) != 0,
            _ => false,
        };
        interrupt_enable
    }
    pub fn step(&mut self, cycles: u8, vram: &[u8], oam: &[u8]) -> u8 {
        let mut interrupt_flag = 0;
        self.cycles += cycles as u32;

        match self.get_mode() {
            PpuMode::OamScan => {
                if self.cycles >= 80 {
                    self.cycles -= 80;
                    self.set_mode(PpuMode::Drawing);
                }
            }
            PpuMode::Drawing => {
                // --- VERSION MOONEYE : Calcul de la durée variable ---
                let mode3_duration = self.calculate_mode3_duration(oam);

                if self.cycles >= mode3_duration {
                    self.cycles -= mode3_duration;
                    self.set_mode(PpuMode::HBlank);
                    if self.ly < 144 {
                        self.draw_scanline(vram, oam);
                    }
                }
            }
            PpuMode::HBlank => {
                let mode3_duration = self.calculate_mode3_duration(oam);
                let mode0_duration = 456 - 80 - mode3_duration; // ScanLine dots - Mode 2 Dots - Mode 3 Dots

                if self.cycles >= mode0_duration {
                    self.cycles -= mode0_duration;
                    self.ly += 1;

                    if self.ly == 144 {
                        self.set_mode(PpuMode::VBlank);
                        interrupt_flag |= 0x01;
                    } else {
                        self.set_mode(PpuMode::OamScan);
                    }
                }
            }
            PpuMode::VBlank => {
                if self.cycles >= 456 {
                    self.cycles -= 456;
                    self.ly += 1;
                    if self.ly > 153 {
                        self.ly = 0;
                        self.set_mode(PpuMode::OamScan);
                    }
                }
            }
        }

        let lyc_int = (self.stat & 0x40) != 0 && (self.ly == self.lyc);
        let mode_bit = self.stat & 0x03;
        let mode_int = match mode_bit {
            0 => (self.stat & 0x08) != 0, // HBlank
            1 => (self.stat & 0x10) != 0, // VBlank
            2 => (self.stat & 0x20) != 0, // OAM
            _ => false,                   // Drawing
        };

        let current_stat_line = lyc_int || mode_int;
        if !self.stat_irq_line && current_stat_line {
            interrupt_flag |= 0x02;
        }
        self.stat_irq_line = current_stat_line;

        if self.ly == self.lyc {
            self.stat |= 0x04;
        } else {
            self.stat &= !0x04;
        }

        interrupt_flag
    }

    fn draw_scanline(&mut self, vram: &[u8], oam: &[u8]) {
        if (self.lcdc & 0x80) == 0 {
            return;
        }
        self.render_bg(vram);
        self.render_sprites(vram, oam);
    }

    fn render_bg(&mut self, vram: &[u8]) {
        if (self.lcdc & 0x01) == 0 {
            return;
        }

        let y_pos = self.scy.wrapping_add(self.ly);
        let tile_row = (y_pos / 8) as u16;

        // VRAM starts at 0x8000 CPU wise. Therefore, 0x1800 is 0x9800
        let map_base = if (self.lcdc & 0x08) != 0 {
            0x1C00
        } else {
            0x1800
        };
        let data_unsigned = (self.lcdc & 0x10) != 0;

        for x in 0..160 {
            let x_pos = self.scx.wrapping_add(x as u8);
            let tile_col = (x_pos / 8) as u16;
            let tile_id = vram[(map_base + (tile_row << 5) + tile_col) as usize];

            let addr = self.get_tile_addr(tile_id, data_unsigned) + ((y_pos & 7) as u16 * 2);
            let color_id = self.get_color_from_bytes(
                vram[addr as usize],
                vram[(addr + 1) as usize],
                x_pos & 7,
            );
            self.set_pixel(x, self.ly, color_id, self.bgp);
        }
    }

    fn render_sprites(&mut self, vram: &[u8], oam: &[u8]) {
        if (self.lcdc & 0x02) == 0 {
            return;
        }
        let height = if (self.lcdc & 0x04) != 0 { 16 } else { 8 };

        // Each Object has 4 bytes (Y, X, Tile, Attributes)
        for i in (0..160).step_by(4) {
            let sprite_y = oam[i] as i16 - 16;
            let sprite_x = oam[i + 1] as i16 - 8;
            let tile_id = if height == 16 {
                oam[i + 2] & 0xFE
            } else {
                oam[i + 2]
            };
            let attr = oam[i + 3];

            if (self.ly as i16) >= sprite_y && (self.ly as i16) < sprite_y + height {
                let mut line = (self.ly as i16 - sprite_y) as u16;
                if (attr & 0x40) != 0 {
                    line = (height as u16 - 1) - line;
                } // Flip Y

                let addr = (tile_id as u16 * 16) + (line * 2);
                let b1 = vram[addr as usize];
                let b2 = vram[(addr + 1) as usize];

                for x in 0..8 {
                    let pixel_x = sprite_x + x;
                    if pixel_x < 0 || pixel_x >= 160 {
                        continue;
                    }

                    let bit = if (attr & 0x20) != 0 { x } else { 7 - x }; // Flip X
                    let color_id = ((b2 >> bit) & 0x01) << 1 | ((b1 >> bit) & 0x01);

                    if color_id != 0 {
                        // Color 0 = Transparent
                        let palette = if (attr & 0x10) != 0 {
                            self.obp1
                        } else {
                            self.obp0
                        };
                        self.set_pixel(pixel_x as u8, self.ly, color_id, palette);
                    }
                }
            }
        }
    }

    // --- Helpers ---
    fn get_tile_addr(&self, id: u8, unsigned: bool) -> u16 {
        if unsigned {
            id as u16 * 16
        } else {
            (0x1000_i16 + (id as i8 as i16 * 16)) as u16
        }
    }

    fn get_color_from_bytes(&self, low: u8, high: u8, bit: u8) -> u8 {
        let bit_idx = 7 - bit;
        ((high >> bit_idx) & 0x01) << 1 | ((low >> bit_idx) & 0x01)
    }

    fn set_pixel(&mut self, x: u8, y: u8, color_id: u8, palette: u8) {
        if y >= 144 {
            return;
        }

        let shade = (palette >> (color_id << 1)) & 0x03;
        let color = match shade {
            0 => [155, 188, 15, 255],
            1 => [139, 172, 15, 255],
            2 => [48, 98, 48, 255],
            3 => [15, 56, 15, 255],
            _ => [0, 0, 0, 255],
        };
        let i = ((y as usize * 160) + x as usize) * 4;
        self.framebuffer[i..i + 4].copy_from_slice(&color);
    }

    fn check_lyc(&mut self) -> bool {
        if self.ly == self.lyc {
            self.stat |= 0x04;
            (self.stat & 0x40) != 0
        } else {
            self.stat &= !0x04;
            false
        }
    }
    fn calculate_mode3_duration(&self, oam: &[u8]) -> u32 {
        let mut duration = 172;

        duration += (self.scx % 8) as u32;

        let sprite_height = if (self.lcdc & 0x04) != 0 { 16 } else { 8 };
        let mut sprites_on_line = 0;

        for i in (0..160).step_by(4) {
            let sprite_y = oam[i] as i16 - 16;
            if (self.ly as i16) >= sprite_y && (self.ly as i16) < sprite_y + sprite_height {
                sprites_on_line += 1;
                duration += 6;
                if sprites_on_line == 10 {
                    break;
                }
            }
        }
        duration
    }
}
