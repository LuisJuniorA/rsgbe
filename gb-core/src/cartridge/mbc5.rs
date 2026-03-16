use crate::cartridge::mbc::MBC;

pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u16, // MBC5 uses a 9-bit ROM bank number
    ram_bank: u8,  // MBC5 supports up to 16 RAM banks
    rom_bank_mask: u16,
    ram_bank_mask: u8,
    ram_enabled: bool,
}

impl MBC5 {
    pub fn new(rom: Vec<u8>, save: Option<Vec<u8>>, ram_size: usize) -> Self {
        let rom_num_banks = rom.len() / 0x4000;
        let rom_bank_mask = if rom_num_banks > 0 {
            (rom_num_banks.next_power_of_two() - 1) as u16
        } else {
            0
        };

        let ram_num_banks = (ram_size / 0x2000) as u8;
        let ram_bank_mask = if ram_num_banks > 0 {
            ram_num_banks.next_power_of_two() - 1
        } else {
            0
        };

        MBC5 {
            rom,
            ram: save.unwrap_or(vec![0; ram_size]),
            rom_bank: 1,
            ram_bank: 0,
            rom_bank_mask,
            ram_bank_mask,
            ram_enabled: false,
        }
    }
}

impl MBC for MBC5 {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            // ROM Bank 00 (Fixed)
            0x0000..=0x3FFF => self.rom[addr as usize],

            // ROM Bank 00-1FF (Switchable)
            0x4000..=0x7FFF => {
                let bank = (self.rom_bank & self.rom_bank_mask) as usize;
                let offset = addr as usize - 0x4000;
                self.rom[bank * 0x4000 + offset]
            }

            // RAM Bank 00-0F (Switchable)
            0xA000..=0xBFFF => {
                if !self.ram_enabled || self.ram.is_empty() {
                    return 0xFF;
                }
                let bank = (self.ram_bank & self.ram_bank_mask) as usize;
                let offset = addr as usize - 0xA000;
                self.ram[bank * 0x2000 + offset]
            }
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => {
                self.ram_enabled = (val & 0x0F) == 0x0A;
            }

            0x2000..=0x2FFF => {
                self.rom_bank = (self.rom_bank & 0x0100) | (val as u16);
            }

            0x3000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0x00FF) | (((val & 0x01) as u16) << 8);
            }

            0x4000..=0x5FFF => {
                self.ram_bank = val & 0x0F;
            }

            0xA000..=0xBFFF => {
                if self.ram_enabled && !self.ram.is_empty() {
                    let bank = (self.ram_bank & self.ram_bank_mask) as usize;
                    let offset = addr as usize - 0xA000;
                    let ram_addr = bank * 0x2000 + offset;
                    if ram_addr < self.ram.len() {
                        self.ram[ram_addr] = val;
                    }
                }
            }
            _ => {}
        }
    }

    fn get_save_data(&self) -> Option<Vec<u8>> {
        if self.ram.is_empty() {
            None
        } else {
            Some(self.ram.clone())
        }
    }
}
