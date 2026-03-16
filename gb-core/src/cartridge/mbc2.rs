use crate::cartridge::mbc::MBC;

pub struct MBC2 {
    rom: Vec<u8>,
    ram: [u8; 512],
    rom_bank: u8,
    rom_bank_mask: u8,
    ram_enabled: bool,
}

impl MBC2 {
    pub fn new(rom: Vec<u8>, save: Option<Vec<u8>>) -> Self {
        let rom_num_banks = rom.len() / 0x4000;
        let rom_bank_mask = if rom_num_banks > 0 {
            (rom_num_banks.next_power_of_two() - 1) as u8
        } else {
            0
        };
        let mut ram = [0; 512];
        if let Some(s) = save {
            let len = s.len().min(512);
            ram[..len].copy_from_slice(&s[..len]);
        }

        MBC2 {
            rom,
            ram: ram,
            rom_bank: 1,
            rom_bank_mask,
            ram_enabled: false,
        }
    }

    pub fn get_effective_rom_bank(&self) -> usize {
        let mut b = self.rom_bank;
        if b == 0 {
            b = 1;
        }
        (b & self.rom_bank_mask) as usize
    }
}

impl MBC for MBC2 {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => {
                let bank = self.get_effective_rom_bank();
                self.rom[bank * 0x4000 + (addr as usize - 0x4000)]
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    return 0xFF;
                }
                self.ram[(addr & 0x01FF) as usize] | 0xF0
            }
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x3FFF => {
                if (addr & 0x0100) == 0 {
                    self.ram_enabled = (val & 0x0F) == 0x0A;
                } else {
                    self.rom_bank = val & 0x0F;
                }
            }
            0xA000..=0xBFFF => {
                if self.ram_enabled {
                    self.ram[(addr & 0x01FF) as usize] = val & 0x0F;
                }
            }
            _ => {}
        }
    }

    fn get_save_data(&self) -> Option<Vec<u8>> {
        if self.ram.is_empty() {
            None
        } else {
            Some(self.ram.to_vec())
        }
    }
}
