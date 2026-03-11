use crate::cartridge::mbc::MBC;

pub struct NoMBC {
    rom: Vec<u8>,
    rom_mask: usize,
}

impl NoMBC {
    pub fn new(rom: Vec<u8>) -> Self {
        let len = rom.len();

        NoMBC {
            rom,
            rom_mask: len - 1,
        }
    }
}

impl MBC for NoMBC {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[(addr as usize) & self.rom_mask],
            0xA000..=0xBFFF => 0xFF, // RAM (even tho there is none)

            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, _addr: u16, _val: u8) {}
}
