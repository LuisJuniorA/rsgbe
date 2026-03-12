use crate::cartridge::mbc::MBC;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>, // 32 KiB max for ram
    rom_bank: u8, // Rom's Bank number (5 bits)
    ram_bank: u8, // Ram's Bank number (2 bits)
    mode: u8,     // 1 bit
    rom_bank_mask: u8,
    ram_bank_mask: u8,
    ram_enabled: bool,
}

impl MBC1 {
    pub fn new(rom: Vec<u8>, ram_size: usize) -> Self {
        let rom_num_banks = rom.len() / 0x4000;
        let rom_bank_mask = if rom_num_banks > 0 {
            (rom_num_banks.next_power_of_two() - 1) as u8
        } else {
            0
        };

        let ram_num_banks = (ram_size / 0x2000).max(1) as u8;
        let ram_bank_mask = ram_num_banks.next_power_of_two() - 1;

        MBC1 {
            rom,
            ram: vec![0; ram_size],
            rom_bank: 1,
            ram_bank: 0,
            mode: 0,
            rom_bank_mask,
            ram_bank_mask,
            ram_enabled: false,
        }
    }

    pub fn get_effective_rom_bank(&self, addr: u16) -> usize {
        let bank = if addr < 0x4000 {
            // 0x0000 - 0x3FFF
            if self.mode == 0 {
                0
            } else {
                self.ram_bank << 5
            }
        } else {
            // 0x4000 - 0x7FFF
            let mut b = self.rom_bank;
            if b == 0 {
                b = 1;
            }
            (self.ram_bank << 5) | b
        };

        (bank & self.rom_bank_mask) as usize
    }

    fn get_effective_ram_bank(&self) -> usize {
        if self.mode == 0 {
            0
        } else {
            (self.ram_bank & self.ram_bank_mask) as usize
        }
    }
}

impl MBC for MBC1 {
    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                let bank = self.get_effective_rom_bank(addr);
                self.rom[bank * 0x4000 + (addr as usize)]
            }
            0x4000..=0x7FFF => {
                let bank = self.get_effective_rom_bank(addr);
                self.rom[bank * 0x4000 + (addr as usize - 0x4000)]
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled || self.ram.is_empty() {
                    return 0xFF;
                }
                let bank = self.get_effective_ram_bank();
                self.ram[bank * 0x2000 + (addr as usize - 0xA000)]
            }
            _ => 0xFF,
        }
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        match addr {
            // Enabling RAM
            0x0000..=0x1FFF => self.ram_enabled = (val & 0xF) == 0x0A,
            // ROM's Bank
            0x2000..=0x3FFF => {
                let mut bank = val & 0x1F;
                if bank == 0 {
                    bank = 1;
                }

                // 5 bits
                self.rom_bank = bank;
            }
            0x4000..=0x5FFF => {
                // 2 bits
                self.ram_bank = val & 0x03;
            }
            0x6000..=0x7FFF => {
                // 1 bit
                self.mode = val & 0x01;
            }
            // Writing in Ram
            0xA000..=0xBFFF => {
                if self.ram_enabled && !self.ram.is_empty() {
                    let bank = self.get_effective_ram_bank();
                    let offset = (addr - 0xA000) as usize;
                    let addr_in_ram = (bank * 0x2000) + offset;

                    if addr_in_ram < self.ram.len() {
                        self.ram[addr_in_ram] = val;
                    }
                }
            }
            _ => {}
        }
    }
}
