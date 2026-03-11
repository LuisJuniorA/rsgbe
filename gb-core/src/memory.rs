const WRAM_SIZE: usize = 1 << 13; // 8192 bytes
const HRAM_SIZE: usize = (1 << 7) - 1; // 127 bytes

pub struct Bus {
    rom: Vec<u8>, // TEMPORARY: flat ROM until we build Cartridge/MBCs
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    ie: u8,
    serial_data: u8,
    pub serial_output: String,
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        Bus {
            rom,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            ie: 0,
            serial_data: 0,
            serial_output: String::new(),
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
            0xFF01 => self.serial_data,
            // $FF80 - $FFFE: High RAM
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
            // Unmapped
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => {} // ROM is read-only
            0x8000..=0x9FFF => {} // TODO: vram
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xFF01 => self.serial_data = val,
            0xFF02 => {
                if val == 0x81 {
                    self.serial_output.push(self.serial_data as char);
                }
            }
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            0xFFFF => self.ie = val,
            _ => {}
        }
    }
}
