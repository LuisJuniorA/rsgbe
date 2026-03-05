const WRAM_SIZE: usize = 1 << 13; // 8192 bytes
const HRAM_SIZE: usize = (1 << 7) - 1; // 127 bytes

pub struct Bus {
    rom: Vec<u8>, // TEMPORARY: flat ROM until we build Cartridge/MBCs
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        Bus {
            rom,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            // vram: todo,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // $0000 - $7FFF: ROM
            0x0000..=0x7FFF => {
                if (addr as usize) < self.rom.len() {
                    self.rom[addr as usize]
                } else {
                    0xFF
                }
            }
            // $C000 - $DFFF: Work RAM
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            // $FF80 - $FFFE: High RAM
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            // Unmapped
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => {} //ROM is read-only
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            _ => {}
        }
    }
}
